
use crate::*;

pub fn expand(lexicon_ident: &syn::Ident, ast_ty: &syn::Type, pt_ty: &syn::Type) -> TokenStream2 {
    let teleparse = crate_ident();

    quote! {
        const _: () = {
            use ::std::vec::Vec;
            use ::std::result::Result;
            use ::std::collections::BTreeSet;
            use ::std::sync::OnceLock;
            use #teleparse::syntax::{FirstBuilder, FollowBuilder, Jump, Metadata};
            use #teleparse::GrammarError;
            #[automatically_derived]
            impl #teleparse::AbstractSyntaxRoot for #ast_ty {
                fn metadata() -> &'static Result<Metadata<#lexicon_ident>, GrammarError>{
                    static METADATA: OnceLock<Result<Metadata<#lexicon_ident>, GrammarError>> 
                        = OnceLock::new();
                    METADATA.get_or_init(|| {
                        let mut first = FirstBuilder::new();
                        Self::build_first(&mut first);
                        let first = first.build();

                        let mut stack = Vec::new();
                        let mut seen = BTreeSet::new();
                        Self::check_left_recursive(&mut stack, &mut seen, &first)?;
                        seen.clear();

                        Self::check_first_conflict(&mut seen, &first)?;
                        seen.clear();

                        let mut follow = FollowBuilder::new(first);
                        Self::build_follow(&mut follow);
                        let (first, follow) = follow.build(<#ast_ty>::type_id());

                        Self::check_first_follow_conflict(&mut seen, &first, &follow)?;
                        seen.clear();

                        let mut jump = Jump::new();
                        Self::build_jump(&mut seen, &first, &mut jump);

                        Ok(Metadata {
                            first,
                            follow,
                            jump
                        })
                    })
                }
            }
            #[automatically_derived]
            impl #teleparse::ParseRoot for #pt_ty {}
        };
    }
}
