//! # teleparse-macros
//!
//! These are for use with the [`teleparse`](https://docs.rs/teleparse) crate and not meant to be used standalone.

mod prelude;
use prelude::*;

mod node_derive_impl;
mod token_type_derive_impl;
mod syntax_tree_derive_impl;

/// Derive macro for traits in the library. Note this is not a normal derive macro, since it also
/// transforms the input in some way
#[proc_macro_attribute]
pub fn teleparse_derive(attr: TokenStream, input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(attr as syn::Ident);
    match teleparse_derive_internal(&ident, input) {
        Ok(out) => out,
        Err(err) => err.into_compile_error().into(),
    }
}

fn teleparse_derive_internal(ident: &syn::Ident, input: TokenStream) -> syn::Result<TokenStream> {
    let out = match ident.to_string().as_str() {
        "TokenType" => {
            token_type_derive_impl::expand(input, ident)
        },
        "Node" => {
            node_derive_impl::expand(input, ident)
        },
        "SyntaxTree" => {
            syntax_tree_derive_impl::expand(input, ident)
        },
        _ => syn_error!(ident, "unknown teleparse_derive input `{}`", ident),
    };

    Ok(out)
}

mod derive_to_span_impl;

/// Derive ToSpan from a type that stores a ToSpan as its first thing
#[proc_macro_derive(ToSpan)]
pub fn derive_to_span(input: TokenStream) -> TokenStream {
    derive_to_span_impl::expand(input)
}

mod derive_root_impl;

/// Derive Root from a SyntaxTree type
#[proc_macro_derive(Root)]
pub fn derive_root(input: TokenStream) -> TokenStream {
    derive_root_impl::expand(input)
}
