//! Implementation for standard library types and utility types

use deref_derive::{Deref, DerefMut};

use crate::{Span, ToSpan};


mod boxed;
// pub mod blanket;
// pub mod iter;
mod option;
pub use option::*;
// mod string;
// pub use string::*;
mod tuple;

#[derive(Deref, DerefMut, ToSpan)]
pub struct Node<T> {
    pub span: Span,
    #[deref]
    pub value: T,
}
pub use teleparse_macros::Node;

impl<T> Node<T> {
    pub fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! ast_passthrough {
    ($ast:tt) => {
        type L=<$ast>::L;
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
            stack: &mut ::std::vec::Vec<std::string::String>,
            set: &mut ::std::collections::BTreeSet<std::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_left_recursive(stack, set, first)
        }
        fn check_first_conflict(
            seen: &mut ::std::collections::BTreeSet<std::any::TypeId>, 
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
            seen: &mut ::std::collections::BTreeSet<std::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
            follow: &$crate::syntax::follow::Follow<Self::L>,
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_first_follow_conflict(seen, first, follow)
        }
        fn build_jump(
            seen: &mut ::std::collections::BTreeSet<std::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
            jump: &mut $crate::syntax::jump::Jump<Self::L>
        ) {
            <$ast>::build_jump(seen, first, jump)
        }
    };
}
pub(crate) use ast_passthrough;
