
pub(crate) use proc_macro::TokenStream;
pub(crate) use proc_macro2::Span;
pub(crate) use quote::quote;
pub(crate) use syn::parse_macro_input;
pub(crate) use syn::punctuated::Punctuated;

/// Type to distinct with proc_macro::TokenStream
pub(crate) type TokenStream2 = proc_macro2::TokenStream;

/// Attribute name, to avoid conflicts with other crates
pub(crate) const CRATE: &str = "llnparse";

pub(crate) fn crate_ident() -> syn::Ident {
    syn::Ident::new(CRATE, Span::call_site())
}

pub(crate) fn parse_attr_meta(attr: &syn::Attribute) -> syn::Result<
Option<Punctuated<syn::Meta, syn::Token![,]>>> {
    if !attr.path().is_ident(CRATE) {
        return Ok(None);
    }

    Ok(Some(attr.parse_args_with(Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated)?))
}

/// Remove [llnparse] attributes from a list of attributes
pub(crate) fn strip_attrs(attrs: &mut Vec<syn::Attribute>) {
    attrs.retain(|attr| !attr.path().is_ident(CRATE));
}

/// Convert a [`syn::Result`] to a [`TokenStream`]
pub(crate) fn from_result(result: syn::Result<TokenStream2>) -> TokenStream {
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

pub(crate) fn from_result_keep_input(input: TokenStream, result: syn::Result<TokenStream2>) -> TokenStream {
    let input2 = TokenStream2::from(input);
    match result {
        Ok(output) => output,
        Err(err) => {
            let output = err.into_compile_error();
            quote! {
                #input2
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
