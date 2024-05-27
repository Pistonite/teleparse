use std::collections::HashSet;

use deref_derive::{Deref, DerefMut};

use crate::{Lexer, Parser, Span, Start, SyntaxResult, SyntaxResultExt, SyntaxTree};

/// A [`SyntaxTree`](crate::SyntaxTree) node where the subtype `T` is not a
/// [`SyntaxTree`](crate::SyntaxTree). This type is used to maintain the span
/// of the subtree
///
/// ## Use cases
/// 
/// 1. The subtree should be converted to another type with `#[llnparse(from(...))]`
/// ```no_compile
/// use llnparse::prelude::*;
///
/// #[llnparse_derive(SyntaxTree)]
/// #[llnparse(token(MyTokenType))]
/// pub struct Foo {
///     #[llnparse(from(Bar))]
///     biz: Node<Biz>,
/// }
///
/// #[llnparse_derive(SyntaxTree)]
/// #[llnparse(token(MyTokenType))]
/// pub struct Bar {
///     ...
/// }
///
/// pub struct Biz {
///     ...
/// }
///
/// impl From<Bar> for Biz {
///    ...
/// }
/// ```
///
/// 2. Extracting info from token while retaining the span
/// ```no_compile
/// use llnparse::prelude::*;
/// #[llnparse_derive(SyntaxTree)]
/// #[llnparse(token(MyTokenType))]
/// pub struct Foo {
///     // x1 will just be the content of the token
///     #[llnparse(token(SomeToken))]
///     x1: String,
///
///     // x2 will also have the span (where that token is at)
///     #[llnparse(token(SomeToken))]
///     x1: Node<String>,
///
///     // y1 will just be if the token exists
///     #[llnparse(token(SomeToken, "match"))]
///     y1: bool,
///
///     // y2 will also have the span (where that token is at)
///     #[llnparse(token(SomeToken, "match"))]
///     y2: Node<bool>,
///
///     // optional also works for strings but not bool
///     #[llnparse(token(SomeToken))]
///     z: Option<String>,
///
///     // this is not valid, as you already have the span
///     #[llnparse(token(SomeToken))]
///     invalid: Node<Span>,
///
///     // it's also invalid without an attribute, as Node itself is not a SyntaxTree
///     not_valid: Node<...>,
/// }
/// ```
#[derive(Deref, DerefMut)]
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
//
// impl<ST: SyntaxTree> SyntaxTree for Node<ST> {
//     type T = ST::T;
//     type Ctx = ST::Ctx;
//
//     #[inline]
//     fn span(&self) -> Span {
//         self.span
//     }
//
//     #[inline]
//     fn start_set() -> HashSet<Start<Self::T>> {
//         ST::start_set()
//     }
//
//     #[inline]
//     fn try_parse<'s, L: Lexer<'s, T=Self::T>>(
//         parser: &mut Parser<'s, Self::T, L, Self::Ctx>) -> SyntaxResult<Self> {
//         ST::try_parse(parser).map_ext(|value| Node::new(value.span(), value))
//     }
//
//     #[inline]
//     fn apply_semantic<'s, L: Lexer<'s, T=Self::T>>(
//         &self, parser: &mut Parser<'s, Self::T, L, Self::Ctx>) {
//         self.value.apply_semantic(parser)
//     }
// }
