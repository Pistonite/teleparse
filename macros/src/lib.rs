//! # teleparse-macros
//!
//! These are for use with the [`teleparse`](https://docs.rs/teleparse) crate and not meant to be used standalone.

mod prelude;
use prelude::*;

mod lexer_derive_impl;
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
        // "Lexer" => {
        //     lexer_derive_impl::expand(input, ident)
        // },
        "SyntaxTree" => {
            syntax_tree_derive_impl::expand(input, ident)
        },
        _ => syn_error!(ident, "unknown teleparse_derive input `{}`", ident),
    };

    Ok(out)
}

// /// Used internally in `llnparse` to ensure that a regex literal is valid
// #[proc_macro]
// pub fn checked_regex_rule(input: TokenStream) -> TokenStream {
//     let regex = {
//         let input = input.clone();
//         parse_macro_input!(input as syn::LitStr)
//     };
//     let result = checked_regex_rule_internal(&regex);
//     from_result_keep_input(quote!{#regex}, result)
// }
//
// fn checked_regex_rule_internal(input: &syn::LitStr) -> syn::Result<TokenStream2> {
//     let regex = input.value();
//     if !regex.starts_with("^") {
//         syn_error!(input, "expected a regular expression starting with `^`, because it always needs to match the beginning of the remaining input");
//     }
//     let regex = match Regex::new(&regex) {
//         Err(e) => {
//             syn_error!(input, e.to_string());
//         },
//         Ok(x) => x
//     };
//     if regex.find("").is_some() {
//         syn_error!(input, "the rule regular expression must not match the empty string");
//     }
//     Ok(quote! { #input })
// }
//
//
