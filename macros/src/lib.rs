use proc_macro::TokenStream;

use quote::quote;

type TokenStream2 = proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn test_my_macro(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let out = quote! {
        #[derive(Debug)]
        #input
    };

    out.into()
}

