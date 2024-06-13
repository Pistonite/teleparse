
#[macro_export]
#[doc(hidden)]
macro_rules! ast_passthrough {
    ($ast:ty) => {
        type L=<$ast as $crate::syntax::AbstractSyntaxTree>::L;
        fn type_id() -> ::core::any::TypeId {
            <$ast>::type_id()
        }
        fn debug() -> ::std::borrow::Cow<'static, str> {
            <$ast>::debug()
        }
        fn build_first(builder: &mut $crate::syntax::first::FirstBuilder<Self::L>) {
            <$ast>::build_first(builder)
        }
        fn check_left_recursive(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            stack: &mut ::std::vec::Vec<::std::string::String>,
            set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_left_recursive(seen, stack, set, first)
        }
        fn check_first_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
            first: &$crate::syntax::first::First<Self::L>
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_first_conflict(seen, first)
        }
        fn build_follow(
            builder: &mut $crate::syntax::follow::FollowBuilder<Self::L>
        ) {
            <$ast>::build_follow(builder)
        }
        fn check_first_follow_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
            follow: &$crate::syntax::follow::Follow<Self::L>,
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_first_follow_conflict(seen, first, follow)
        }
        fn build_jump(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
            jump: &mut $crate::syntax::jump::Jump<Self::L>
        ) {
            <$ast>::build_jump(seen, first, jump)
        }
    };
}

