
use crate::*;

pub fn expand_internal(input: &syn::DeriveInput) -> syn::Result<TokenStream2> {
    let teleparse = crate_ident();
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let out = quote! {
        #[automatically_derived]
        impl #impl_generics #teleparse::Root for #ident #ty_generics #where_clause {
            #teleparse::derive_root_impl!{#ident}
        }
    };

    Ok(out)
}
