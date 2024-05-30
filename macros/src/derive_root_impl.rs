
use crate::*;

pub fn expand(input: TokenStream) -> TokenStream {
    let mut derive_input = {
        let input = input.clone();
        parse_macro_input!(input as syn::DeriveInput)
    };
    let result = expand_internal(&mut derive_input);
    from_result_keep_input(quote!{#derive_input}, result)
}

fn expand_internal(input: &syn::DeriveInput) -> syn::Result<TokenStream2> {
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
