
use quote::ToTokens;

use crate::*;

pub fn expand<A: ToTokens, B: ToTokens>(ast_ty: A, pt_ty: B) -> TokenStream2 {
    let teleparse = crate_ident();

    quote! {
        #[automatically_derived]
        impl #teleparse::AbstractSyntaxRoot for #ast_ty {
            fn metadata(
            ) -> &'static ::core::result::Result<
                #teleparse::syntax::Metadata<Self::L>, #teleparse::GrammarError
            > {
                use #teleparse::syntax::AbstractSyntaxTree;
                static METADATA: ::std::sync::OnceLock<
                    ::core::result::Result<
                        #teleparse::syntax::Metadata<<#ast_ty as #teleparse::syntax::AbstractSyntaxTree>::L>, #teleparse::GrammarError
                >> = ::std::sync::OnceLock::new();
                METADATA.get_or_init(|| {
                    let _lexer = <Self::L as #teleparse::lex::Lexicon>::lexer("")?;

                    let mut first = #teleparse::syntax::FirstBuilder::new();
                    Self::build_first(&mut first);
                    let (names, first) = first.build();

                    let mut stack = ::std::vec::Vec::new();
                    let mut seen = ::std::collections::BTreeSet::new();
                    let mut set = ::std::collections::BTreeSet::new();
                    Self::check_left_recursive(&mut seen, &mut stack, &mut set, &first)?;
                    seen.clear();

                    Self::check_first_conflict(&mut seen, &first)?;
                    seen.clear();

                    let mut follow = #teleparse::syntax::FollowBuilder::new(first);
                    Self::build_follow(&mut follow);
                    let (first, follow) = follow.build(<#ast_ty>::type_id());

                    Self::check_first_follow_conflict(&mut seen, &first, &follow)?;
                    seen.clear();

                    let mut jump = #teleparse::syntax::Jump::new();
                    Self::build_jump(&mut seen, &first, &mut jump);

                    Ok(#teleparse::syntax::Metadata {
                        names,
                        first,
                        follow,
                        jump
                    })
                })
            }
        }
        #[automatically_derived]
        impl #teleparse::ParseRoot for #pt_ty {}
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

            use #teleparse::AbstractSyntaxRoot;

            #[test]
            fn is_ll1() {
                assert!(<#pt_ty as #teleparse::parser::ParseTree>::AST::metadata().is_ok(), "Grammar is not LL(1)");
            }
        }
    }
}
