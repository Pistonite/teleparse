use crate::*;

pub fn expand(input: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match &input.data {
        syn::Data::Union(_) => {
            syn_error!(input, "Union is not supported for ToSpan");
        }
        syn::Data::Enum(data) => {
            expand_enum(data)?
        }
        syn::Data::Struct(data) => {
            expand_struct(data)?
        }
    };

    let out = quote! {
        #[automatically_derived]
        impl #impl_generics #teleparse::ToSpan for #ident #ty_generics #where_clause {
            fn span(&self) -> #teleparse::Span {
                #body
            }
        }
    };

    Ok(out)
}

fn expand_struct(input: &syn::DataStruct) -> syn::Result<TokenStream2> {
    match &input.fields {
        syn::Fields::Named(fields) => {
            let ident = match fields.named.first() {
                Some(syn::Field { ident: Some(id), .. }) => id,
                _ => {
                    return unsupported_empty_field(&input.fields);
                }
            };
            Ok(quote! {
                self.#ident.span()
            })
        }
        syn::Fields::Unnamed(_) => {
            Ok(quote! {
                self.0.span()
            })
        }
        syn::Fields::Unit => {
            unsupported_empty_field(&input.fields)
        }
    }
}

fn expand_enum(input: &syn::DataEnum) -> syn::Result<TokenStream2> {
    let mut arms = TokenStream2::new();
    for variant in &input.variants {
        let ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field = match fields.named.first() {
                    Some(syn::Field { ident: Some(id), .. }) => id,
                    _ => {
                        return unsupported_empty_field(&variant.fields);
                    }
                };
                arms.extend(quote! {
                    Self::#ident { #field, .. } => #field.span(),
                });
            }
            syn::Fields::Unnamed(_) => {
                arms.extend(quote! {
                    Self::#ident(x, ..) => x.span(),
                });
            }
            syn::Fields::Unit => {
                unsupported_empty_field(&variant.fields)?;
            }
        }
    }

    Ok(quote! {
        match self {
            #arms
        }
    })
}

fn unsupported_empty_field(fields: &syn::Fields) -> syn::Result<TokenStream2> {
    syn_error!(fields, "Must have at least one field to derive ToSpan");
}
