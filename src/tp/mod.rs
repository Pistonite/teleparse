//! Implementation for standard library types and utility types

use deref_derive::{Deref, DerefMut};

use crate::{Span, ToSpan};


// pub mod blanket;
pub mod iter;
pub mod option;
pub mod string;
pub use string::*;

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
    fn build_start_table(
        s_table: &mut crate::table::SyntaxTreeTable<Self::T>,
        lits: &mut crate::table::LitTable) -> bool{
        ST::build_start_table(s_table, lits)
        }

    #[inline]
    fn build_follow_table<'s>(
        s_table: &'s crate::table::SyntaxTreeTable<Self::T>, 
        f_table: &mut crate::table::SyntaxTreeTable<Self::T>,
        follows: &crate::table::TermSet<Self::T>
        ) -> (std::borrow::Cow<'s, crate::table::TermSet<Self::T>>, bool) {
        ST::build_follow_table(s_table, f_table, follows)
        }
    #[inline]
    fn try_parse_ast<'s>(parser: &mut crate::Parser<'s, Self::T>
        , f_table: &crate::table::SyntaxTreeTable<Self::T>, should_recover: bool) -> crate::SyntaxResult<Self::T, Self::AST> {
        ST::try_parse_ast(parser, f_table, should_recover)
    }
    };
}
pub(crate) use ast_passthrough;
