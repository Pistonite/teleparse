//! Blanket implementation for std types in SyntaxTree

use std::collections::HashSet;

use crate::{Lexer, Parser, Span, Start, SyntaxResult, SyntaxResultExt, SyntaxTree, SyntaxTreeParser};

pub struct BlanketParser<ST>(std::marker::PhantomData<ST>);
impl<'s, ST: SyntaxTree> SyntaxTreeParser<'s> for BlanketParser<ST> {
    type T = ST::T;
    type Ctx = ST::Ctx;
    type Target = ST;

    #[inline]
    fn try_parse<L: Lexer<'s, T = Self::T>>(
        &self,
        parser: &mut Parser<
            's,
            Self::T,
            L,
            Self::Ctx,
        >,
    ) -> SyntaxResult<Self::Target> {
        ST::try_parse(parser)
    }
}

impl<ST: SyntaxTree> SyntaxTree for Box<ST> {
    type T = ST::T;
    type Ctx = ST::Ctx;

    #[inline]
    fn span(&self) -> Span {
        self.as_ref().span()
    }

    #[inline]
    fn start_set() -> HashSet<Start<Self::T>> {
        ST::start_set()
    }

    #[inline]
    fn try_parse<'s, L: Lexer<'s, T=Self::T>>(
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>) -> SyntaxResult<Self> {
        ST::try_parse(parser).map_ext(Box::new)
    }

    #[inline]
    fn apply_semantic<'s, L: Lexer<'s, T=Self::T>>(
        &self, parser: &mut Parser<'s, Self::T, L, Self::Ctx>) {
        self.as_ref().apply_semantic(parser)
    }
}
