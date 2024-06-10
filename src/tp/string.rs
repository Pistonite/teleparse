//! string-based syntax tree nodes ([`Quote`], [`Parse`], [`ParseDefault`])
use std::marker::PhantomData;
use std::str::FromStr;
use std::string::String as StdString;

use crate::{ToSpan, Parser, ParseTree, AbstractSyntaxTree};

use super::Node;

/// Node that stores stringified source code
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Quote<S, T: ParseTree>(Node<S>, PhantomData<T>);

impl<S: std::fmt::Debug, T: ParseTree> std::fmt::Debug for Quote<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<S, T: ParseTree> ParseTree for Quote<S, T> 
    where S: for <'a> From<&'a str>
{
    type AST = T::AST;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        let span = ast.span();
        let _ = T::from_ast(ast, parser);
        let src = parser.info().get_src(span);
        Node::new(span, S::from(src)).into()
    }
}

/// Alias for `Quote<String, T>`
pub type String<T> = Quote<StdString, T>;

/// Node that stores a parsed value from a string or the error if parse failed
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Parse<S: FromStr, T: ParseTree>(Node<Result<S, S::Err>>, PhantomData<T>);
impl<S: FromStr + std::fmt::Debug, T: ParseTree> std::fmt::Debug for Parse<S, T> 
    where S::Err: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<S: FromStr, T: ParseTree> ParseTree for Parse<S, T> 
{
    type AST = T::AST;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        let span = ast.span();
        let _ = T::from_ast(ast, parser);
        let src = parser.info().get_src(span);
        Node::new(span, S::from_str(src)).into()
    }
}
/// Parse-or-default. Node that stores a parsed value from a string or the default value if parse failed
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct ParseDefault<S: FromStr + Default, T: ParseTree>(Node<S>, PhantomData<T>);
impl<S: FromStr + std::fmt::Debug + Default, T: ParseTree> std::fmt::Debug for ParseDefault<S, T> 
    where S::Err: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<S: FromStr + Default, T: ParseTree> ParseTree for ParseDefault<S, T> {
    type AST = T::AST;
    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        let span = ast.span();
        let _ = T::from_ast(ast, parser);
        let src = parser.info().get_src(span);
        Node::new(span, S::from_str(src).unwrap_or_default()).into()
    }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::GrammarError;
    use crate::ParseTree;
    use crate::tp::Node;

    use crate::lex::Token;
    use crate::test::prelude::*;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd, Integer};

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct Stringified(super::String<Ident>);

    #[test]
    fn test_stringify() {
        let t = Stringified::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..1 => \"a\"");
        assert_eq!(t, Stringified(Node::new(0..1, "a".to_string()).into()));
    }

    #[test]
    fn test_deref_string() {
        let t = Stringified::parse("a").unwrap().unwrap();
        let x: &String = &t.0;
        assert_eq!(x, "a");
        assert_eq!(&*t.0, "a");
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct Parsed {
        ident: super::Parse<u32, Ident>,
        num: super::Parse<u32, Integer>,
        float: super::Parse<f32, Integer>,
        ident_default: super::ParseDefault<u32, Ident>,
    }

    #[test]
    fn test_parse() {
        let t = Parsed::parse("abc 456 314 def").unwrap().unwrap();
        assert!(t.ident.is_err());
        assert_eq!(t.num, Node::new(4..7, Ok(456)).into());
        assert_eq!(t.float, Node::new(8..11, Ok(314.0)).into());
        assert_eq!(t.ident_default, Node::new(12..15, 0).into());

        assert_eq!(*t.num, Ok(456));
        assert_eq!(*t.float, Ok(314.0));
        assert_eq!(*t.ident_default, 0);
    }
}
