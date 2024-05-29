//! Implementation for standard library types and utility types

use deref_derive::{Deref, DerefMut};

use crate::{Span, ToSpan};


// pub mod blanket;
pub mod iter;
// pub mod option;
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

macro_rules! ast_passthrough {
    () => {
    type T=ST::T;
    type AST=ST::AST;
    #[inline]
    fn try_parse_ast<'s>(parser: &mut Parser<'s, Self::T>) -> SyntaxResult<Self::AST> {
        ST::try_parse_ast(parser)
    }
    };
}
pub(crate) use ast_passthrough;
