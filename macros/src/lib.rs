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
pub fn derive_syntax(_: TokenStream, input: TokenStream) -> TokenStream {
    expand_with_mut(input, syntax::expand)
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

// internal helpers

mod root;
