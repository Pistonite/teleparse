use regex::Regex;

pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::Span;
pub(crate) use quote::{quote, format_ident};
pub(crate) use syn::parse_macro_input;
pub(crate) use syn::punctuated::Punctuated;

/// Type to distinct with proc_macro::TokenStream
pub(crate) type TokenStream2 = proc_macro2::TokenStream;

/// Attribute name, to avoid conflicts with other crates
pub(crate) const CRATE: &str = "teleparse";

pub(crate) fn crate_ident() -> syn::Ident {
    match proc_macro_crate::crate_name(CRATE) {
        Ok(proc_macro_crate::FoundCrate::Name(s)) => syn::Ident::new(&s, Span::call_site()),
        _ => syn::Ident::new(CRATE, Span::call_site()),
    }
}

pub(crate) fn expand_with<F>(input: TokenStream, f: F)  -> TokenStream
where F:
    FnOnce(&syn::DeriveInput) -> syn::Result<TokenStream2>
{
    expand_with_args(input, (), |input, _| f(input))
}

pub(crate) fn expand_with_args<A, F>(input: TokenStream, args: A, f: F)  -> TokenStream
where F:
    FnOnce(&syn::DeriveInput, A) -> syn::Result<TokenStream2>
{
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let result = f(&derive_input, args);
    from_result_keep_input(quote!{#derive_input}, result)
}


pub(crate) fn expand_with_mut<F>(input: TokenStream, f: F)  -> TokenStream
where F:
    FnOnce(&mut syn::DeriveInput) -> syn::Result<TokenStream2>
{
    expand_with_args_mut(input, (), |input, _| f(input))
}

pub(crate) fn expand_with_args_mut<A, F>(input: TokenStream, args: A, f: F)  -> TokenStream
where F:
    FnOnce(&mut syn::DeriveInput, A) -> syn::Result<TokenStream2>
{
    let mut derive_input = {
        let input = input.clone();
        parse_macro_input!(input as syn::DeriveInput)
    };
    let result = f(&mut derive_input, args);
    from_result_keep_input(quote!{#derive_input}, result)
}

pub(crate) fn parse_crate_attr_meta(attr: &syn::Attribute) -> syn::Result<
Punctuated<syn::Meta, syn::Token![,]>> {
    attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)
}

/// Take teleparse attributes from a list of attributes
pub(crate) fn strip_take_attrs(attrs: &mut Vec<syn::Attribute>) -> Vec<syn::Attribute> {
    let take_attrs = {
        let mut out = Vec::with_capacity(attrs.len());
        std::mem::swap(&mut out, attrs);
        out
    };
    let mut out = Vec::new();
        for attr in take_attrs {
        if !attr.path().is_ident(CRATE) {
            // put it back
            attrs.push(attr);
        } else {
            out.push(attr);
        }
    }
    out
}

pub(crate) fn from_result_keep_input(input: TokenStream2, result: syn::Result<TokenStream2>) -> TokenStream {
    match result {
        Ok(output) => output,
        Err(err) => {
            let output = err.into_compile_error();
            quote! {
                #input
                #output
            }
        }
    }.into()
}

/// Macro for creating and returning `syn::Error`
macro_rules! syn_error {
    ($tokens:expr, $msg:expr) => {
        return Err(syn::Error::new_spanned($tokens, $msg))
    };
    ($tokens:expr, $($tt:tt)*) => {
        return Err(syn::Error::new_spanned($tokens, format!($($tt)*)))
    };
}
pub(crate) use syn_error;

/// Parse the `teleparse` attribute on the input and return the meta list, also stripping the attribute
pub(crate) fn parse_strip_root_meta_optional(input: &mut syn::DeriveInput) -> syn::Result<Option<Punctuated<syn::Meta, syn::Token![,]>>> {
    let root_attrs = strip_take_attrs(&mut input.attrs);
    let root_attr = match ensure_one(root_attrs) {
        EnsureOne::None => return Ok(None),
        EnsureOne::More => syn_error!(&input.ident, "Multiple root {} attributes found! You might want to merge them.", CRATE),
        EnsureOne::One(attr) => attr,
    };
    Ok(Some(parse_crate_attr_meta(&root_attr)?))
}

#[must_use]
pub(crate) fn ensure_one<I: IntoIterator>(input: I) -> EnsureOne<I::Item> {
    let mut iter = input.into_iter();
    match iter.next() {
        None => EnsureOne::None,
        Some(first) => match iter.next() {
            None => EnsureOne::One(first),
            Some(_) => EnsureOne::More,
        }
    }
}

pub(crate) enum EnsureOne<T> {
    None,
    One(T),
    More,
}

pub(crate) fn checked_regex_rule(input: &syn::LitStr) -> syn::Result<Regex> {
    let regex = input.value();
    let regex = match Regex::new(&regex) {
        Err(e) => {
            syn_error!(input, e.to_string());
        },
        Ok(x) => x
    };
    if regex.find("").is_some() {
        syn_error!(input, "the rule regular expression must not match the empty string");
    }
    Ok(regex)
}

/// Put a block in an anonymous const block
pub(crate) fn anon_const_block(block: TokenStream2) -> TokenStream2 {
    quote! {
        const _: () = {
            #block
        };
    }
}
