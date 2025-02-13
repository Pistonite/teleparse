use quote::ToTokens;

use crate::*;

pub fn expand<T: ToTokens>(pt_ty: T) -> TokenStream2 {
    let teleparse = crate_ident();

    quote! {
        #[automatically_derived]
        impl #teleparse::parser::Root for #pt_ty {
            fn metadata(
            ) -> &'static ::core::result::Result<
                #teleparse::syntax::Metadata<<Self::Prod as #teleparse::syntax::Production>::L>,
                #teleparse::GrammarError
            > {
                use #teleparse::syntax::Production;
                static METADATA: ::std::sync::OnceLock<
                    ::core::result::Result<
                        #teleparse::syntax::Metadata<
                            <<#pt_ty as #teleparse::parser::Produce>::Prod as #teleparse::syntax::Production>::L
                        >,
                        #teleparse::GrammarError
                    >
                > = ::std::sync::OnceLock::new();
                METADATA.get_or_init(|| {
                    #teleparse::syntax::Metadata::build_for::<<#pt_ty as #teleparse::parser::Produce>::Prod>()
                })
            }
        }
    }
}

pub fn expand_test(pt_ty: &syn::Ident) -> TokenStream2 {
    let mod_name = format_ident!("{}_root_test", pt_ty);

    let teleparse = crate_ident();

    quote! {
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod #mod_name {
            use super::*;

            #[test]
            fn is_ll1() {
                use #teleparse::Root;
                #pt_ty::assert_ll1()
            }
        }
    }
}
