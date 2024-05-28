
pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::Span;
pub(crate) use quote::quote;
pub(crate) use syn::parse_macro_input;
pub(crate) use syn::punctuated::Punctuated;

/// Type to distinct with proc_macro::TokenStream
pub(crate) type TokenStream2 = proc_macro2::TokenStream;

/// Attribute name, to avoid conflicts with other crates
pub(crate) const CRATE: &str = "teleparse";

pub(crate) fn crate_ident() -> syn::Ident {
    syn::Ident::new(CRATE, Span::call_site())
}

pub(crate) fn parse_attr_meta(attr: &syn::Attribute) -> syn::Result<
Option<Punctuated<syn::Meta, syn::Token![,]>>> {
    if !attr.path().is_ident(CRATE) {
        return Ok(None);
    }

    Ok(Some(parse_crate_attr_meta(attr)?))
}

pub(crate) fn parse_crate_attr_meta(attr: &syn::Attribute) -> syn::Result<
Punctuated<syn::Meta, syn::Token![,]>> {
    Ok(attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)?)
}

/// Remove [llnparse] attributes from a list of attributes
pub(crate) fn strip_attrs(attrs: &mut Vec<syn::Attribute>) {
    attrs.retain(|attr| !attr.path().is_ident(CRATE));
}

/// Take [llnparse] attributes from a list of attributes
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

/// Convert a [`syn::Result`] to a [`TokenStream`]
pub(crate) fn from_result(result: syn::Result<TokenStream2>) -> TokenStream {
    result.unwrap_or_else(syn::Error::into_compile_error).into()
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
pub(crate) fn parse_strip_root_meta(input: &mut syn::DeriveInput, derive_ident: &syn::Ident) -> syn::Result<Punctuated<syn::Meta, syn::Token![,]>> {
    let context = derive_ident.to_string();
    let root_attrs = strip_take_attrs(&mut input.attrs);
    let root_attr = match ensure_one(root_attrs) {
        EnsureOne::None => syn_error!(derive_ident, "Deriving {} requires a {} attribute to define additional properties.", context, CRATE),
        EnsureOne::More => syn_error!(derive_ident, "Multiple root {} attributes found for {}! You might want to merge them.", CRATE, context),
        EnsureOne::One(attr) => attr,
    };
    parse_crate_attr_meta(&root_attr)
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
    syn::parse_quote! { () }
}
