use crate::{
    token::ToSpan, Parser, Span, SyntaxError, SyntaxErrorKind, SyntaxResult, Token, TokenStorage, TokenType, TokenTypeNoCtx
};
use std::{collections::HashSet, ops::Deref};

type CtxOf<T> = <T as TokenType>::Ctx;

pub trait SyntaxTree: Sized + ToSpan {
    type T: TokenType;
    type AST: ToSpan;

    fn parse_with_context( source: &str, context: CtxOf<Self::T>) -> (Option<Self>, CtxOf<Self::T>) {
        let mut parser = Self::T::parser_with_context(source, context);
        let result = parser.next();
        (result, parser.context)
    }

    fn parse_all_with_context( source: &str, context: CtxOf<Self::T>) -> (Vec<Self>, CtxOf<Self::T>) {
        let mut parser = Self::T::parser_with_context(source, context);
        let result = parser.parse_all();
        (result, parser.context)
    }

    // fn span_of(ast: &Self::AST) -> Span;
    //
    // fn start_set() -> HashSet<Start<Self::T>>;

    /// Attempt to parse one AST node
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn try_parse_ast<'s>(parser: &mut Parser<'s, Self::T>) -> SyntaxResult<Self::AST>;

    /// Transform the parsed AST node into the final tree node
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self;
}

pub trait SyntaxTreeNoCtx: SyntaxTree {
    fn parse(source: &str) -> Option<Self>;
    
    fn parse_all(source: &str) -> Vec<Self>;
}

impl<T: TokenTypeNoCtx, AST, ST: SyntaxTree<T=T, AST=AST>> SyntaxTreeNoCtx for ST {
    #[inline]
    fn parse(source: &str) -> Option<Self> {
        let (result, _) = Self::parse_with_context(source, ());
        result
    }
    
    #[inline]
    fn parse_all(source: &str) -> Vec<Self> {
        let (result, _) = Self::parse_all_with_context(source, ());
        result
    }
}

// #[derive(Debug, Hash)]
// pub enum Start<T: TokenType> {
//     Epsilon,
//     Token(T),
//     TokenMatch(T, String),
// }
//
// pub trait SyntaxTreeParser<'s> {
//     type T: TokenType;
//     type Ctx;
//     type Target;
//
//     fn try_parse<L: Lexer<'s, T = Self::T>>(
//         &self,
//         parser: &mut Parser<
//             's,
//             Self::T,
//             Self::Ctx,
//         >,
//     ) -> SyntaxResult<Self::Target>;
// }
