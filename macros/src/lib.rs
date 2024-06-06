//! # teleparse-macros
//!
//! These are for use with the [`teleparse`](https://docs.rs/teleparse) crate and not meant to be used standalone.

mod prelude;
use prelude::*;


/// Transform an enum into a token type (a lexicon)
///
/// This will derive the lexicon trait as well as the super traits, and also generate
/// an implementation for the lexer, and implementation for terminal symbols for the AST
///
/// Note that this is not a derive macro, since it will transform the input.
#[proc_macro_attribute]
pub fn derive_lexicon(_: TokenStream, input: TokenStream) -> TokenStream {
    expand_with_mut(input, lexicon::expand)
}
mod lexicon;

/// Transform an enum or struct into a parse tree node, as well as deriving the production rule
/// (the AST nodes)
///
/// This will derive the AbstractSyntaxTree trait as well as the super traits, and also generate
/// an implementation for the lexer, and implementation for terminal symbols for the AST
///
/// Note that this is not a derive macro, since it will transform the input.
#[proc_macro_attribute]
pub fn derive_syntax(attr: TokenStream, input: TokenStream) -> TokenStream {
    let lexicon_ident = parse_macro_input!(attr as syn::Ident);
    expand_with_args_mut(input, lexicon_ident, syntax::expand)
}
mod syntax;

/// Derive common traits for AST helper nodes (stores a Node as its first thing)
#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    expand_with(input, node::expand)
}
mod node;

/// Derive ToSpan from a type that stores a ToSpan as its first thing
#[proc_macro_derive(ToSpan)]
pub fn derive_to_span(input: TokenStream) -> TokenStream {
    expand_with(input, to_span::expand)
}
mod to_span;

mod root;

// /// Derive Root from a SyntaxTree type
// #[proc_macro_derive(Root)]
// pub fn derive_root(input: TokenStream) -> TokenStream {
//     derive_root_impl::expand(input)
// }
//
// mod derive_ll1_impl;
//
// /// Derive LL1 test for a Root type
// #[proc_macro_derive(LL1Test)]
// pub fn derive_ll1(input: TokenStream) -> TokenStream {
//     derive_ll1_impl::expand(input)
// }
