//! Implementation for standard library types and utility types

use deref_derive::{Deref, DerefMut};

use crate::{Span, ToSpan};


// pub mod blanket;
pub mod iter;
mod option;
pub use option::*;
mod string;
pub use string::*;
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
    type T=ST::T;
    type AST=ST::AST;

        #[inline]
        fn can_be_empty() -> bool {
        ST::can_be_empty()
        }

        #[inline]
        fn check_left_recursive(stack: &mut std::vec::Vec<std::any::TypeId>, set: &mut std::collections::BTreeSet<std::any::TypeId>) -> bool {
        ST::check_left_recursive(stack, set)
        }

    #[inline]
    fn build_first_table(
        s_table: &mut crate::table::SyntaxTreeTable<Self::T>,
        lits: &mut crate::table::LitTable) {
        ST::build_first_table(s_table, lits)
        }

        #[inline]
        fn has_first_collision(first: &crate::table::SyntaxTreeTable<Self::T>) -> bool {
        ST::has_first_collision(first)
        }

    #[inline]
    fn build_follow_table(
        first: &crate::table::SyntaxTreeTable<Self::T>, 
        follow: &mut crate::table::SyntaxTreeTable<Self::T>,
        ) -> bool {
        ST::build_follow_table(first, follow)
        }
    #[inline]
    fn try_parse_ast<'s>(parser: &mut crate::Parser<'s, Self::T>
        , f_table: &crate::table::SyntaxTreeTable<Self::T>, should_recover: bool) -> crate::SyntaxResult<Self::T, Self::AST> {
        ST::try_parse_ast(parser, f_table, should_recover)
    }
    };
}
pub(crate) use ast_passthrough;
