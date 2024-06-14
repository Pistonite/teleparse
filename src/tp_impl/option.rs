//! optional syntax tree nodes ([`Option`], [`Exists`])
use std::marker::PhantomData;


use crate::parser::ParseTree;
use crate::{AbstractSyntaxTree, Parser, ToSpan};

use super::{Node, OptionAST};

/// Node that stores an optional subtree `Option<T> => T | epsilon`
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Optional<T: ParseTree>(Node<Option<T>>);

impl<T: std::fmt::Debug + ParseTree> std::fmt::Debug for Optional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0.value {
            Some(node) => {
                f.debug_tuple("Some").field(&node).finish()
            }
            None => {
                f.debug_tuple("None").field(&self.0.span).finish()
            }
        }
    }
}

impl<T: ParseTree> ParseTree for Optional<T> {
    type AST = OptionAST<T::AST>;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        match ast {
            OptionAST::Some(ast) => {
                let t = T::from_ast(ast, parser);
                Node::new(t.span(), Some(t)).into()
            }
            OptionAST::None(span) => {
                Node::new(span, None).into()
            }
        }
    }
}

/// Node that stores if an optional subtree is produced
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Exists<T: ParseTree>(Node<bool>, PhantomData<T>);

impl<T: ParseTree> std::fmt::Debug for Exists<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ParseTree> ParseTree for Exists<T> {
    type AST = OptionAST<T::AST>;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        match ast {
            OptionAST::Some(ast) => {
                let t = T::from_ast(ast, parser);
                Node::new(t.span(), true).into()
            }
            OptionAST::None(span) => {
                Node::new(span, false).into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::GrammarError;

    use crate::lex::Token;
    use crate::test::prelude::*;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd};

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct OptIdent(tp::Option<Ident>);

    #[test]
    fn test_none() {
        let t = OptIdent::parse("+").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "None(0)");
        assert_eq!(t, OptIdent(Node::new(0..0, None).into()));
    }

    #[test]
    fn test_some() {
        let t = OptIdent::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "Some(token Ident(0..1))");
        assert_eq!(t, OptIdent(Node::new(0..1, Some(
            Ident(Token::new(0..1, T::Ident))
        )).into()));
    }

    #[test]
    fn test_use_as_option() {
        let t = OptIdent::parse("+").unwrap().unwrap();
        assert!(t.0.is_none());
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    struct Seq(tp::Option<OpAdd>, OpAdd);

    #[test]
    fn test_seq_not_ll1() {
        assert_not_ll1!(Seq, GrammarError::FirstFollowSeqConflict(
            "Seq".to_string(),
            "Option<OpAdd>".to_string(),
            "OpAdd".to_string(),
            "\"+\"".to_string()
        ));
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    struct Nested(super::Optional<super::Optional<Ident>>);

    #[test]
    fn test_nested_not_ll1() {
        assert_not_ll1!(Nested, GrammarError::FirstFirstConflict(
            "Option<Option<Ident>>".to_string(),
            "Option<Ident>".to_string(),
            "<epsilon>".to_string(),
        ));
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct ExistIdent(tp::Exists<Ident>);

    #[test]
    fn parse_exist() {
        let t = ExistIdent::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..1 => true");
        assert_eq!(t, ExistIdent(Node::new(0..1, true).into()));
        assert_eq!(*t.0, true);

        let t = ExistIdent::parse("+").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0 => false");
        assert_eq!(t, ExistIdent(Node::new(0..0, false).into()));
        assert_eq!(*t.0, false);
    }
}
