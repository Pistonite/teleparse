use std::marker::PhantomData;

use deref_derive::{Deref, DerefMut};

use crate::{Lexer, Parser, ParserState, Span, SyntaxResult, SyntaxResultExt, SyntaxTree, SyntaxTreeParser, Token, TokenType};

use crate::prelude::*;

/// Parser for things marked with `#[llnparse(token(...))]`
// pub struct TokenParser<T: TokenType, Ctx, Out> {
//     pub ty: T,
//     pub match_lit: Option<&'static str>,
//     _phantom: PhantomData<(Ctx, Out)>,
// }
//
// impl<T: TokenType, Ctx, Out> TokenParser<T, Ctx, Out> {
//     #[inline]
//     pub fn new(ty: T) -> Self {
//         Self {
//             ty,
//             match_lit: None,
//             _phantom: PhantomData,
//         }
//     }
//     #[inline]
//     pub fn with_match_lit(ty: T, match_lit: &'static str) -> Self {
//         Self {
//             ty,
//             match_lit: Some(match_lit),
//             _phantom: PhantomData,
//         }
//     }
// }
//
// impl<T: TokenType, Ctx, Out> TokenParser<T, Ctx, Out> {
//     fn parse_internal
//     <'s, L: Lexer<'s, T=T>, >
//     (&self, parser: &mut Parser<'s, T, L, Ctx>) -> SyntaxResult<Token<T>> {
//         let token = match parser.consume_token() {
//             Some(token) => token,
//             None => return parser.unexpected_eof().into(),
//         };
//         if token.ty == self.ty {
//             if let Some(match_lit) = self.match_lit {
//                 if parser.get_src(&token) == match_lit {
//                     return Ok(token);
//                 }
//             }
//             else {
//                 return Ok(token);
//             }
//         }
//         token.unexpected().into()
//     }
// }
// #[derive(Deref, DerefMut)]
// pub struct TokenNode<T: TokenType, Ctx>(#[deref] Token<T>, PhantomData<Ctx>);
//
// impl<T: TokenType, Ctx> SyntaxTree for TokenNode<T, Ctx> {
//     type T = T;
//     type Ctx = Ctx;
//     type AST = Token<T>;
//
//     #[inline]
//     fn span_of(ast: &Self::AST) -> Span {
//         ast.span
//     }
//
//     fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
//         _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
//     ) -> SyntaxResult<Self::AST> {
//         panic!("try_parse_ast for token should never be called - the derive macro should generated code that avoids calling it.")
//     }
//
//     #[inline]
//     fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
//         ast: Self::AST,
//         _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
//     ) -> Self {
//         TokenNode(ast, PhantomData)
//     }
// }


#[inline(never)]
pub fn parse_token<'s,T: TokenType
    , L: Lexer<'s, T=T>, Ctx
>(
    ty: T,
    parser: &mut Parser<'s, T, L, Ctx>,
) -> SyntaxResult<Token<T>> {
    let token = match parser.consume_token() {
        Some(token) => token,
        None => return parser.unexpected_eof().into(),
    };
    if token.ty == ty {
        return Ok(token);
    }
    token.unexpected().into()
}

#[inline(never)]
pub fn parse_token_match<'s,T: TokenType
    , L: Lexer<'s, T=T>, Ctx
>(
    ty: T,
match_lit: &'static str,
    parser: &mut Parser<'s, T, L, Ctx>,
) -> SyntaxResult<Token<T>> {
    let token = match parser.consume_token() {
        Some(token) => token,
        None => return parser.unexpected_eof().into(),
    };
    if token.ty == ty {
                if parser.get_src(&token) == match_lit {
                    return Ok(token);
                }
    }
    token.unexpected().into()
}

// impl<'s, T: TokenType, Ctx> SyntaxTreeParser<'s> for TokenParser<T, Ctx, Token<T>> {
//     type T = T;
//     type Ctx = Ctx;
//     type Target = Token<T>;
//     #[inline]
//     fn try_parse<L: Lexer<'s, T=T>>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         self.parse_internal(parser)
//     }
// }
//
// impl<'s, T: TokenType, Ctx> SyntaxTreeParser<'s> for TokenParser<T, Ctx, Span> {
//     type T = T;
//     type Ctx = Ctx;
//     type Target = Span;
//     #[inline]
//     fn try_parse<L: Lexer<'s, T=T>>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         self.parse_internal(parser).map_ext(|t| t.span)
//     }
// }
// impl<'s, T: TokenType, Ctx> SyntaxTreeParser<'s> for TokenParser<T, Ctx, Option<Span>> {
//     type T = T;
//     type Ctx = Ctx;
//     type Target = Option<Span>;
//     fn try_parse<L: Lexer<'s, T=T>>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         self.parse_optional(parser).map_ext(|t| t.map(|t|t.span))
//     }
// }
// impl<'s, T: TokenType, Ctx, Out: From<&'s str>> SyntaxTreeParser<'s> for TokenParser<T, Ctx, Out> {
//     type T = T;
//     type Ctx = Ctx;
//     type Target = Out;
//     #[inline]
//     fn try_parse<L: Lexer<'s, T=T>>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         self.parse_internal(parser).map_ext(|t| Out::from(parser.get_src(&t)))
//     }
// }
// impl<'s, T: TokenType> ParseTokenSyntax<'s> for TokenParser<T, Option<&'s str>> {
//     type T = T;
//     type Out = Option<&'s str>;
//     fn parse<L: Lexer<'s, T=T>, Ctx>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Option<&'s str>> {
//         self.parse_optional(parser).map_ext(|t| t.map(|t| parser.get_src(&t)))
//     }
// }
// impl<'s, T: TokenType, Ctx> SyntaxTreeParser<'s> for TokenParser<T, Ctx, bool> {
//     type T = T;
//     type Ctx = Ctx;
//     type Target = bool;
//     fn try_parse<L: Lexer<'s, T=T>>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         self.parse_optional(parser).map_ext(|t| t.is_some())
//     }
// }
// impl<'s, T: TokenType> ParseTokenSyntax<'s> for TokenParser<T, Node<&'s str>> {
//     type T = T;
//     type Out = Node<&'s str>;
//     fn parse<L: Lexer<'s, T=T>, Ctx>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Node<&'s str>> {
//         self.parse_internal(parser).map_ext(|t| Node::new(t.span, parser.get_src(&t)))
//     }
// }
// impl<'s, T: TokenType> ParseTokenSyntax<'s> for TokenParser<T, Option<Node<&'s str>>> {
//     type T = T;
//     type Out = Option<Node<&'s str>>;
//     fn parse<L: Lexer<'s, T=T>, Ctx>(
//         &self, parser: &mut Parser<'s, T, L, Ctx>,
//     ) -> SyntaxResult<Option<Node<&'s str>>> {
//         self.parse_optional(parser).map_ext(|t| t.map(|t| Node::new(t.span, parser.get_src(&t))))
//     }
// }
    
