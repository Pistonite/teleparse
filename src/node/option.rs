use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use deref_derive::{Deref, DerefMut};

use crate::{parser::ParserState, ToSpan, Lexer, Parser, Span, SyntaxResult, SyntaxResultExt, SyntaxTree};

use super::Node;

pub struct Optional<ST: SyntaxTree>(Node<Option<ST>>);

impl<ST: SyntaxTree> Optional<ST> {
    #[inline]
    pub fn new(span: Span, value: Option<ST>) -> Self {
        Self(Node::new(span, value))
    }
}

impl<ST: SyntaxTree> ToSpan for Optional<ST> {
    #[inline]
    fn span(&self) -> Span {
        self.0.span
    }
}

impl<ST: SyntaxTree> SyntaxTree for Optional<ST> {
    type T = ST::T;
    type AST = Result<ST::AST, Span>;

    fn try_parse_ast<'s>(
        parser: &mut Parser<'s, Self::T>
    ) -> SyntaxResult<Self::AST> {
        parser.push_state().map_ext(|_| {
            let result = ST::try_parse_ast(parser).into_value();
            match result {
                Some(ast) => {
                    parser.pop_state();
                    Ok(ast)
                }
                None => {
                    let e = Err(parser.current_span());
                    parser.restore_state();
                    parser.pop_state();
                    e
                }
            }
        })
    }

    #[inline]
    fn into_parse_tree<'s>(
        ast: Self::AST,
        parser: &mut Parser<'s, Self::T>
    ) -> Self {
        match ast {
            Ok(ast) => Self::new(ast.span(), Some(ST::into_parse_tree(ast, parser))),
            Err(span) => Self(Node::new(span, None)),
        }
    }
}

impl<ST: SyntaxTree> Deref for Optional<ST> {
    type Target=Option<ST>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}

impl<ST: SyntaxTree> DerefMut for Optional<ST> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.value
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_optional_as_option() {
        let o = Optional(Node::new(Span::new(0, 0), Some(42)));
        assert_eq!(o.as_ref().copied(), Some(42));
        let opt: &Option<i32> = &o;
        assert_eq!(opt.as_ref().copied(), Some(42));
    }
    #[test]
    fn use_exists() {
        let e = Exists::<String>::new(Span::new(0, 0), true);
        assert!(e.exists());
    }
}
