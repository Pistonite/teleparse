use crate::*;

pub fn expand(input: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data = match &input.data {
        syn::Data::Struct(data) => data,
        _ => {
            syn_error!(input, "Node can only be derived for structs")
        }
    };

    let mut from_body = TokenStream2::new();
    let mut field_expr = TokenStream2::new();
    let mut target_ty = TokenStream2::new();

    match &data.fields {
        syn::Fields::Named(fields) => {
            let (ident, ty) = match fields.named.first() {
                Some(syn::Field {
                    ident: Some(id),
                    ty,
                    ..
                }) => (id, ty),
                _ => {
                    return unsupported_empty_field(&data.fields);
                }
            };
            let ty = parse_node_type(ty)?;
            from_body = quote! { Self { #ident, ::core::default::Default::default() } };
            field_expr = quote! { self.#ident };
            target_ty = quote! { #ty };
        }
        syn::Fields::Unnamed(fields) => {
            let ty = match fields.unnamed.first() {
                Some(syn::Field { ty, .. }) => ty,
                _ => {
                    return unsupported_empty_field(&data.fields);
                }
            };
            let ty = parse_node_type(ty)?;

            from_body.extend(quote! { node });
            for _ in 1..fields.unnamed.len() {
                from_body.extend(quote! { , ::core::default::Default::default() });
            }
            from_body = quote! { Self(#from_body) };
            field_expr = quote! { self.0 };
            target_ty = quote! { #ty };
        }
        syn::Fields::Unit => {
            unsupported_empty_field(&data.fields)?;
        }
    }

    let out = quote! {
        #[automatically_derived]
        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn into_inner(self) -> #target_ty {
                #field_expr.value
            }
        }
        #[automatically_derived]
        impl #impl_generics ::core::convert::From<#teleparse::tp::Node<#target_ty>> for #ident #ty_generics #where_clause {
            #[inline]
            fn from(node: #teleparse::tp::Node<#target_ty>) -> Self {
                #from_body
            }
        }
        #[automatically_derived]
        impl #impl_generics ::core::ops::Deref for #ident #ty_generics #where_clause {
            type Target = #target_ty;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &#field_expr.value
            }
        }
        #[automatically_derived]
        impl #impl_generics ::core::ops::DerefMut for #ident #ty_generics #where_clause {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut #field_expr.value
            }
        }
    };

    Ok(out)
}

fn parse_node_type(ty: &syn::Type) -> syn::Result<TokenStream2> {
    let path = match ty {
        syn::Type::Path(ty) => &ty.path,
        _ => {
            syn_error!(ty, "Expected a path")
        }
    };

    match path.segments.last() {
        Some(syn::PathSegment {
            arguments: syn::PathArguments::AngleBracketed(inner),
            ..
        }) => {
            let args = &inner.args;
            Ok(quote! { #args })
        }
        _ => {
            syn_error!(path, "Expected Node type with generic")
        }
    }
}

fn unsupported_empty_field<T>(fields: &syn::Fields) -> syn::Result<T> {
    syn_error!(fields, "Must have at least one field to derive Node");
}
