use crate::{
    lexer::Lexer, Parser, Pos, Span, SrcToken, SyntaxError, SyntaxErrorKind, SyntaxResult, Token,
    TokenStorage, TokenType,
};
use std::{collections::HashSet, ops::Deref};

pub trait SyntaxTree: Sized {
    type T: TokenType;
    type Ctx;
    type AST;

    fn parse_with_context<'s, L: Lexer<'s, T = Self::T>>(
        source: &'s str,
        context: Self::Ctx,
    ) -> (Option<Self>, Self::Ctx) {
        let mut parser = Parser::<'s, Self::T, L, Self::Ctx>::new_with_context(source, context);
        let result = parser.next();
        (result, parser.context)
    }

    fn parse_all_with_context<'s, L: Lexer<'s, T = Self::T>>(
        source: &'s str,
        context: Self::Ctx,
    ) -> (Vec<Self>, Self::Ctx) {
        let mut parser = Parser::<'s, Self::T, L, Self::Ctx>::new_with_context(source, context);
        let result = parser.parse_all();
        (result, parser.context)
    }

    fn span_of(ast: &Self::AST) -> Span;
    //
    // fn start_set() -> HashSet<Start<Self::T>>;

    /// Attempt to parse one AST node
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> SyntaxResult<Self::AST>;

    /// Transform the parsed AST node into the final tree node
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
        ast: Self::AST,
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> Self;
}

pub trait SyntaxTreeNoCtx: SyntaxTree<Ctx = ()> {
    fn parse<'s, L: Lexer<'s, T = Self::T>>(source: &'s str) -> Option<Self> {
        let (result, _) = Self::parse_with_context::<L>(source, ());
        result
    }

    fn parse_all<'s, L: Lexer<'s, T = Self::T>>(source: &'s str) -> Vec<Self> {
        let (result, _) = Self::parse_all_with_context::<L>(source, ());
        result
    }
}

#[derive(Debug, Hash)]
pub enum Start<T: TokenType> {
    Epsilon,
    Token(T),
    TokenMatch(T, String),
}

pub trait SyntaxTreeParser<'s> {
    type T: TokenType;
    type Ctx;
    type Target;

    fn try_parse<L: Lexer<'s, T = Self::T>>(
        &self,
        parser: &mut Parser<
            's,
            Self::T,
            L,
            Self::Ctx,
        >,
    ) -> SyntaxResult<Self::Target>;
}
