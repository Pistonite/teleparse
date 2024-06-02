use crate::*;

pub fn expand(input: TokenStream) -> TokenStream {
    let mut derive_input = {
        let input = input.clone();
        parse_macro_input!(input as syn::DeriveInput)
    };
    let result = expand_internal(&mut derive_input);
    from_result_keep_input(quote!{#derive_input}, result)
}

fn expand_internal(input: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();
    let ident = &input.ident;

    let out = quote! {
        #[automatically_derived]
        #[cfg(test)]
        mod root_test {
            use super::*;

            #[test]
            fn is_ll1() {
                #teleparse::assert_ll1!(#ident);
            }
        }
    };

    Ok(out)
}
