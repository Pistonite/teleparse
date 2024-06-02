//! Implementation for standard library types and utility types

use deref_derive::{Deref, DerefMut};

use crate::{Span, ToSpan};


mod ptr;
pub use ptr::*;
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
    // type T=ST::T;
    // type AST=ST::AST;

        #[inline]
        fn type_id() -> std::any::TypeId {
            ST::type_id()
        }

        #[inline]
        fn debug() -> std::borrow::Cow<'static, str> {
            ST::debug()
        }

        #[inline]
        fn produces_epsilon() -> bool {
        ST::produces_epsilon()
        }

        #[inline]
        fn check_left_recursive(
        stack: &mut std::vec::Vec<std::string::String>,
        set: &mut std::collections::BTreeSet<std::any::TypeId>
        ) -> Result<(), $crate::LL1Error> {
        ST::check_left_recursive(stack, set)
        }

    #[inline]
    fn build_first(builder: &mut crate::table::first::FirstBuilder<Self::T>) {
        ST::build_first(builder)
        }

        #[inline]
        fn check_first_conflict(
        seen: &mut std::collections::BTreeSet<std::any::TypeId>, 
        first: &crate::table::first::First<Self::T>) -> Result<(), $crate::LL1Error> {
            ST::check_first_conflict(seen, first)
        }

        #[inline]
        fn check_first_conflict_recursive(
        seen: &mut std::collections::BTreeSet<std::any::TypeId>, 
        first: &crate::table::first::First<Self::T>) -> Result<(), $crate::LL1Error> {
            ST::check_first_conflict_recursive(seen, first)
        }
    #[inline]
    fn build_follow(builder: &mut $crate::table::follow::FollowBuilder<Self::T>) {
        ST::build_follow(builder)
        }

    #[inline]
    fn check_first_follow_conflict(
        seen: &mut std::collections::BTreeSet<std::any::TypeId>,
        first: &crate::table::first::First<Self::T>,
        follow: &crate::table::follow::Follow<Self::T>,
        ) -> Result<(), $crate::LL1Error> {
            ST::check_first_follow_conflict(seen, first, follow)
        }
    #[inline]
    fn check_first_follow_conflict_recursive(
        seen: &mut std::collections::BTreeSet<std::any::TypeId>,
        first: &crate::table::first::First<Self::T>,
        follow: &crate::table::follow::Follow<Self::T>,
        ) -> Result<(), $crate::LL1Error> {
            ST::check_first_follow_conflict_recursive(seen, first, follow)
        }
        #[inline]
        fn build_parsing(seen: &mut std::collections::BTreeSet<std::any::TypeId>, parsing: &mut $crate::table::parsing::Parsing<Self::T>) {
            ST::build_parsing(seen, parsing)
        }
    // #[inline]
    // fn try_parse_ast<'s>(parser: &mut crate::Parser<'s, Self::T>
    //     , f_table: &crate::table::SyntaxTreeTable<Self::T>, should_recover: bool) -> crate::SyntaxResult<Self::T, Self::AST> {
    //     ST::try_parse_ast(parser, f_table, should_recover)
    // }
    };
}
pub(crate) use ast_passthrough;
