use std::marker::PhantomData;

//
use crate::{Lexer, Parser, ParserState, Span, SyntaxResult, SyntaxResultExt, SyntaxTree, SyntaxTreeParser};

use super::node::Node;

pub struct Text<'a, T: From<&'a str>>(Node<T>, PhantomData<&'a str>);
impl<'a, T: From<&'a str>> Text<'a, T> {
    pub fn new(span: Span, text: &'a str) -> Self {
        Self(Node::new(span, text.into()), PhantomData)
    }
}
