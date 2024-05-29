
pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::Span;
pub(crate) use quote::quote;
use regex::Regex;
pub(crate) use syn::{parse_macro_input, parse_quote};
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

pub(crate) fn parse_crate_attr_meta(attr: &syn::Attribute) -> syn::Result<
Punctuated<syn::Meta, syn::Token![,]>> {
    Ok(attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)?)
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

/// Parse the `llnparse` attribute on the input and return the meta list, also stripping the attribute
pub(crate) fn parse_strip_root_meta_optional(input: &mut syn::DeriveInput, derive_ident: &syn::Ident) -> syn::Result<Option<Punctuated<syn::Meta, syn::Token![,]>>> {
    let context = derive_ident.to_string();
    let root_attrs = strip_take_attrs(&mut input.attrs);
    let root_attr = match ensure_one(root_attrs) {
        EnsureOne::None => return Ok(None),
        EnsureOne::More => syn_error!(derive_ident, "Multiple root {} attributes found for {}! You might want to merge them.", CRATE, context),
        EnsureOne::One(attr) => attr,
    };
    Ok(Some(parse_crate_attr_meta(&root_attr)?))
}

pub(crate) fn parse_strip_root_meta(input: &mut syn::DeriveInput, derive_ident: &syn::Ident) -> syn::Result<Punctuated<syn::Meta, syn::Token![,]>> {
    match parse_strip_root_meta_optional(input, derive_ident)? {
        Some(metas) => Ok(metas),
        None => {
            let context = derive_ident.to_string();
            syn_error!(derive_ident, "Deriving {} requires a {} attribute to define additional properties.", context, CRATE)
        }
    }
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

pub(crate) fn get_doc(attrs: &[syn::Attribute]) -> TokenStream2 {
    attrs.iter().fold(TokenStream2::new(), |mut acc, attr| {
        if attr.path().is_ident("doc") {
            acc.extend(quote! { #attr });
        }
        acc
    })
}

pub(crate) fn unit_type() -> syn::Type {
    parse_quote! { () }
}

pub(crate) fn is_unit_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Tuple(ty) => ty.elems.is_empty(),
        _ => false,
    }
}

pub(crate) fn checked_regex_rule(input: &syn::LitStr) -> syn::Result<Regex> {
    let regex = input.value();
    if !regex.starts_with("^") {
        syn_error!(input, "expected a regular expression starting with `^`, because it always needs to match the beginning of the remaining input");
    }
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
