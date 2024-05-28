//! Blanket implementation for std types in SyntaxTree

use std::{collections::HashSet, marker::PhantomData};

use crate::{Lexer, Parser, Span, Start, SyntaxResult, SyntaxResultExt, SyntaxTree, SyntaxTreeParser};

// pub struct BlanketParser<ST>(PhantomData<ST>);
// impl<ST> BlanketParser<ST> {
//     pub fn new() -> Self {
//         Self(PhantomData)
//     }
// }
// impl<'s, ST: SyntaxTree> SyntaxTreeParser<'s> for BlanketParser<ST> {
//     type T = ST::T;
//     type Ctx = ST::Ctx;
//     type Target = ST;
//
//     #[inline]
//     fn try_parse<L: Lexer<'s, T = Self::T>>(
//         &self,
//         parser: &mut Parser<
//             's,
//             Self::T,
//             L,
//             Self::Ctx,
//         >,
//     ) -> SyntaxResult<Self::Target> {
//         ST::try_parse(parser)
//     }
// }

impl<ST: SyntaxTree> SyntaxTree for Box<ST> {
    type T = ST::T;
    type Ctx = ST::Ctx;
    type AST = Box<ST::AST>;

    #[inline]
    fn span_of(ast: &Self::AST) -> Span {
        ST::span_of(ast.as_ref())
    }

    // #[inline]
    // fn start_set() -> HashSet<Start<Self::T>> {
    //     ST::start_set()
    // }

    #[inline]
    fn try_parse_ast<'s, L: Lexer<'s, T=Self::T>>(
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>) -> SyntaxResult<Self::AST> {
        ST::try_parse_ast(parser).map_ext(Box::new)
    }

    #[inline]
    fn into_parse_tree<'s, L: Lexer<'s, T=Self::T>>(
        ast: Self::AST, parser: &mut Parser<'s, Self::T, L, Self::Ctx>) -> Self {
        Box::new(ST::into_parse_tree(*ast, parser))
    }
}
