use crate::*;

/// Derive the `TokenType` trait for an enum and generate variant representation
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    from_result(expand_internal(input))
}

fn expand_internal(input: syn::DeriveInput) -> syn::Result<TokenStream2> {
    // has to be enum
    let enum_data = match &input.data {
        syn::Data::Enum(data) => data,
        _ => syn_error!(input, "TokenType can only be derived for enums")
    };
    if !input.generics.params.is_empty() {
        syn_error!(input, "TokenType derive cannot be used with generics")
    }
    let (repr, repr_str) = match enum_data.variants.len() {
        0 => syn_error!(input, "TokenType derive cannot be used with enums with no variants"),
        1..=8 => (quote!(u8), "u8"),
        9..=16 => (quote!(u16), "u16"),
        17..=32 => (quote!(u32), "u32"),
        33..=64 => (quote!(u64), "u64"),
        65..=128 => (quote!(u128), "u128"),
        _ => syn_error!(input, "TokenType derive can have at most 128 variants")
    };
    let mut x = 1u128;
    let mut enum_body = TokenStream2::new();
    let mut should_extract_match_clauses = TokenStream2::new();
    for variant in &enum_data.variants {
        if !matches!(variant.fields, syn::Fields::Unit) {
            syn_error!(variant, "TokenType derive must be used with enums with only unit variants, as integer representation will be generated");
        }

        let ident = &variant.ident;
        let num = syn::LitInt::new(&format!("0x{x:x}{repr_str}"), Span::call_site());
        enum_body.extend(quote! {
            #ident = #num,
        });
        x <<= 1;

        // check for attributes
        let mut extract = false;
        for attr in &variant.attrs {
            if !is_crate_attr(attr) {
                continue;
            }
            let id = attr.parse_args::<syn::Ident>()?;
            if id == "extract" {
                extract = true;
            } else {
                syn_error!(id, "unknown {CRATE} attribute `{id}`");
            }
        }

        if extract {
            should_extract_match_clauses.extend(quote! {
                Self::#ident => true,
            });
        }
        
    }
    let enum_name = &input.ident; 
    let enum_vis = &input.vis;
    let enum_first_ident = &enum_data.variants.first().unwrap().ident;
    let enum_last_ident = &enum_data.variants.last().unwrap().ident;
    let llnparse = crate_ident();
    let out = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(#repr)]
        #enum_vis enum #enum_name {
            #enum_body
        }
        #[automatically_derived]
        impl #llnparse::TokenType for #enum_name {
            type Repr = #repr;
            #[inline]
            fn should_extract(&self) -> bool {
                match self {
                    #should_extract_match_clauses
                    _ => false
                }
            }
            #[inline]
            fn to_repr(&self) -> Self::Repr {
                *self as Self::Repr
            }
            #[inline]
            fn first() -> Self {
                Self::#enum_first_ident
            }
            fn next(&self) -> Option<Self> {
                match self {
                    Self::#enum_last_ident => None,
                    _ => {
                        let repr = self.to_repr();
                        let next = repr << 1;
                        Some(unsafe { std::mem::transmute(next) })
                    }
                }
            }
        }
    };

    Ok(out)
}
