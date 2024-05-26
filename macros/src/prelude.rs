
pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::Span;
pub(crate) use quote::quote;
pub(crate) use syn::parse_macro_input;

/// Type to distinct with proc_macro::TokenStream
pub(crate) type TokenStream2 = proc_macro2::TokenStream;

/// Attribute name, to avoid conflicts with other crates
pub(crate) const CRATE: &str = "llnparse";

pub(crate) fn crate_ident() -> syn::Ident {
    syn::Ident::new(CRATE, Span::call_site())
}

/// Check if an attribute is a crate attribute
pub(crate) fn is_crate_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident(CRATE)
}

/// Convert a [`syn::Result`] to a [`TokenStream`]
pub(crate) fn from_result(result: syn::Result<TokenStream2>) -> TokenStream {
    result.unwrap_or_else(syn::Error::into_compile_error).into()
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
