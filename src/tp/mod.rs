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

impl<T: ToSpan> ToSpan for Result<T, Span> {
    #[inline]
    fn span(&self) -> Span {
        match self {
            Ok(value) => value.span(),
            Err(span) => *span,
        }
    }
}

/// Implement AST parsing as a pass through.
///
/// This means
/// - The AST type is the same as the inner AST type
/// - The AST has the same start and follow set as the inner AST
macro_rules! ast_passthrough {
    () => {
        type L=AST::L;

        #[inline]
        fn type_id() -> std::any::TypeId {
            AST::type_id()
        }

        #[inline]
        fn debug() -> std::borrow::Cow<'static, str> {
            AST::debug()
        }
        #[inline]
        fn build_first(builder: &mut $crate::syntax::first::FirstBuilder<Self::L>) {
            AST::build_first(builder)
        }

        #[inline]
        fn check_left_recursive(
            stack: &mut std::vec::Vec<std::string::String>,
            set: &mut std::collections::BTreeSet<std::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
        ) -> Result<(), $crate::GrammarError> {
            AST::check_left_recursive(stack, set, first)
        }


        #[inline]
        fn check_first_conflict(
            seen: &mut std::collections::BTreeSet<std::any::TypeId>, 
            first: &crate::syntax::first::First<Self::L>
        ) -> Result<(), $crate::GrammarError> {
            AST::check_first_conflict(seen, first)
        }

        #[inline]
        fn build_follow(
            builder: &mut $crate::syntax::follow::FollowBuilder<Self::L>
        ) {
            AST::build_follow(builder)
        }

        #[inline]
        fn check_first_follow_conflict(
            seen: &mut std::collections::BTreeSet<std::any::TypeId>,
            first: &crate::syntax::first::First<Self::L>,
            follow: &crate::syntax::follow::Follow<Self::L>,
        ) -> Result<(), $crate::GrammarError> {
            AST::check_first_follow_conflict(seen, first, follow)
        }

        #[inline]
        fn build_jump(
            seen: &mut std::collections::BTreeSet<std::any::TypeId>,
            first: &crate::syntax::first::First<Self::L>,
            jump: &mut $crate::syntax::jump::Jump<Self::L>
        ) {
            AST::build_jump(seen, first, jump)
        }
    };
}
pub(crate) use ast_passthrough;
