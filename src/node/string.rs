
//! string-based syntax tree nodes ([`Quote`], [`Parse`], [`Pod`])
use std::marker::PhantomData;
use std::str::FromStr;

use crate::prelude::*;
use crate::parser::ParserState;
use crate::{Parser, SyntaxResult, SyntaxTree};

use super::{ast_passthrough, Node};

/// Node that stores stringified source code
#[teleparse_derive(Node)]
pub struct Quote<S: From<String>, ST: SyntaxTree>(Node<S>, PhantomData<ST>);

/// Alias for `Quote<String, ST>`
pub type QuoStr<ST> = Quote<String, ST>;

impl<S: From<String>, ST: SyntaxTree> SyntaxTree for Quote<S, ST> {
    ast_passthrough!();

    #[inline]
    fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self {
        let span = ast.span();
        let src: S = parser.get_src_span(span).to_string().into();
        ST::into_parse_tree(ast, parser);
        Node::new(span, src).into()
    }
}

/// Node that stores a parsed value from a string or the error if parse failed
#[teleparse_derive(Node)]
pub struct Parse<S: FromStr, ST: SyntaxTree>(Node<Result<S, S::Err>>, PhantomData<ST>);

impl<S: FromStr, ST: SyntaxTree> SyntaxTree for Parse<S, ST> {
    ast_passthrough!();

    #[inline]
    fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self {
        let span = ast.span();
        ST::into_parse_tree(ast, parser);
        let src = parser.get_src_span(span);
        Node::new(span, src.parse()).into()
    }
}

/// Parse-or-default. Node that stores a parsed value from a string or the default value if parse failed
#[teleparse_derive(Node)]
pub struct Pod<S: FromStr + Default, ST: SyntaxTree>(Node<S>, PhantomData<ST>);

impl<S: FromStr + Default, ST: SyntaxTree> SyntaxTree for Pod<S, ST> {
    ast_passthrough!();

    #[inline]
    fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self {
        let span = ast.span();
        ST::into_parse_tree(ast, parser);
        let src = parser.get_src_span(span);
        let s: S = src.parse().unwrap_or_default();
        Node::new(span, s).into()
    }
}

