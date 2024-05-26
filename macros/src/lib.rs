//! # llnparse-macros
//!
//! These are for use with the [`llnparse`](https://docs.rs/llnparse) crate and not meant to be used standalone.

mod prelude;
use prelude::*;

mod lexer_derive_impl;
mod token_type_derive_impl;

/// Derive trait from llnparse along with other required traits
#[proc_macro_attribute]
pub fn llnparse_derive(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as syn::Ident);
    match llnparse_derive_internal(&ident, input) {
        Ok(out) => out,
        Err(err) => err.into_compile_error().into(),
    }
}

fn llnparse_derive_internal(ident: &syn::Ident, input: TokenStream) -> syn::Result<TokenStream> {
    let out = match ident.to_string().as_str() {
        "TokenType" => {
            token_type_derive_impl::expand(input)
        },
        "Lexer" => {
            lexer_derive_impl::expand(input)
        },
        _ => syn_error!(ident, "unknown llnparse_derive input `{ident}`"),
    };

    Ok(out)
}


