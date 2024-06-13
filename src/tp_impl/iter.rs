
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::marker::PhantomData;

use crate::syntax::{First, FirstBuilder, FirstRel, Follow, FollowBuilder, FollowRel, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, ParseTree, Parser, Span, ToSpan};

use super::{Node, OneOrMore, OptionAST};


#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Plus<V: FromIterator<T>, T: ParseTree>(Node<V>, PhantomData<T>);

impl<V: FromIterator<T> + std::fmt::Debug, T: ParseTree> std::fmt::Debug for Plus<V, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<V: FromIterator<T>, T: ParseTree> ParseTree for Plus<V, T> {
    type AST = OneOrMore<T::AST>;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        let span = ast.span();
        let v: V = ast.0.into_iter().map(|t| T::from_ast(t, parser)).collect();
        Node::new(span, v).into()
    }
}

#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Star<V: FromIterator<T> + Default, T: ParseTree>(Node<V>, PhantomData<T>);

impl<V: FromIterator<T> + Default + std::fmt::Debug, T: ParseTree> std::fmt::Debug for Star<V, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<V: FromIterator<T> + Default, T: ParseTree> ParseTree for Star<V, T> {
    type AST = OptionAST<OneOrMore<T::AST>>;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        let span = ast.span();
        let v: V = match ast {
            OptionAST::Some(ast) =>
    ast.0.into_iter().map(|t| T::from_ast(t, parser)).collect(),
            OptionAST::None(_) => Default::default(),
        };

        Node::new(span, v).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::syntax::{ErrorKind, FirstSet};
    use crate::{syntax, Parser, GrammarError};

    use crate::test::prelude::*;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd};

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct IdentList(tp::Plus<Vec<Ident>, Ident>);

    #[test]
    fn parse_single_item() {
        let t = IdentList::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..1 => [token Ident(0..1)]");
        assert_eq!(t, IdentList(Node::new(0..1, vec![
            Ident::from_span(0..1)
        ]).into()));
    }

    #[test]
    fn parse_two_items() -> Result<(), GrammarError> {
        let mut parser = Parser::<T>::new("a b")?;
        let t = parser.parse::<IdentList>()?.unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..3 => [token Ident(0..1), token Ident(2..3)]");
        assert_eq!(t, IdentList(Node::new(0..3, vec![
            Ident::from_span(0..1),
            Ident::from_span(2..3),
        ]).into()));

        Ok(())
    }

    #[test]
    fn parse_many() {
        let t = IdentList::parse("a b c  d e").unwrap().unwrap();
        assert_eq!(t, IdentList(Node::new(0..10, vec![
            Ident::from_span(0..1),
            Ident::from_span(2..3),
            Ident::from_span(4..5),
            Ident::from_span(7..8),
            Ident::from_span(9..10),
        ]).into()));
    }

    #[test]
    fn parse_with_panic() -> Result<(), GrammarError> {
        let mut parser = Parser::<T>::new("a b c+e")?;
        let root = parser.parse::<IdentList>()?;

        assert_eq!(parser.remaining(), "+e");
        assert_eq!(root, Some(
            IdentList(Node::new(0..5, vec![
                Ident::from_span(0..1),
                Ident::from_span(2..3),
                Ident::from_span(4..5),
            ]).into())),
        );

        assert_eq!(parser.parse::<IdentList>()?, None);
        assert_eq!(parser.remaining(), "+e");

        parser.consume_token();
        let root = parser.parse::<IdentList>()?;

        assert_eq!(root, Some(
            IdentList(Node::new(6..7, vec![
                Ident::from_span(6..7),
            ]).into()),
        ));
        

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct ListPlusList(tp::Plus<Vec<Ident>, Ident>, OpAdd, tp::Plus<Vec<Ident>, Ident>);

    #[test]
    fn parse_single_item_each() {
        let t = ListPlusList::parse("a+ b").unwrap().unwrap();
        let t_str = format!("{:?}", t);
        assert_eq!(t_str, "ListPlusList(0..1 => [token Ident(0..1)], token Op(1..2), 3..4 => [token Ident(3..4)])");
        assert_eq!(t.span(), Span::from(0..4));
        assert_eq!(t, ListPlusList(
            Node::new(0..1, vec![Ident::from_span(0..1)]).into(),
            OpAdd::from_span(1..2),
            Node::new(3..4, vec![Ident::from_span(3..4)]).into(),
        ));
    }

    #[test]
    fn parse_multiple_items_each() {
        let t = ListPlusList::parse("a b c + b c d").unwrap().unwrap();
        assert_eq!(t, ListPlusList(
            Node::new(0..5, vec![
                Ident::from_span(0..1),
                Ident::from_span(2..3),
                Ident::from_span(4..5),
            ]).into(),
            OpAdd::from_span(6..7),
            Node::new(8..13, vec![
                Ident::from_span(8..9),
                Ident::from_span(10..11),
                Ident::from_span(12..13),
            ]).into(),
        ));
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq, Clone)]
    struct ListOfOption(tp::Plus<Vec<tp::Option<Ident>>, tp::Option<Ident>>);

    #[test]
    fn list_of_option_not_ll1() {
        assert_not_ll1!(ListOfOption, GrammarError::FirstFollowSeqConflict(
            "OneOrMore<Option<Ident>>".to_string(),
            "Option<Ident>".to_string(),
            "Option<OneOrMore<Option<Ident>>>".to_string(),
            "<epsilon>".to_string()
        ));
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq, Clone)]
    struct SameList(tp::Nev<OpAdd>, tp::Nev<OpAdd>);

    #[test]
    fn two_list_of_same_not_ll1() {
        assert_not_ll1!(SameList, GrammarError::FirstFollowConflict(
            "Option<OneOrMore<OpAdd>>".to_string(),
            "\"+\"".to_string(),
        ));
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct SeparatedList(Ident, tp::Nev<(OpAdd, Ident)>);

    #[test]
    fn parse_separated_list() {
        let t = SeparatedList::parse("a + b + c").unwrap().unwrap();
        assert_eq!(t, SeparatedList(
            Ident::from_span(0..1),
            Node::new(2..9, vec![
                (OpAdd::from_span(2..3), Ident::from_span(4..5)),
                (OpAdd::from_span(6..7), Ident::from_span(8..9)),
            ]).into(),
        ));
    }

    #[test]
    fn parse_separated_list_multiple() -> Result<(), GrammarError> {
        let mut parser = Parser::<T>::new("a  +b c+e")?;
        let root = parser.parse::<SeparatedList>()?;

        assert_eq!(parser.remaining(), "c+e");
        assert_eq!(root, Some(
            SeparatedList(
                Ident::from_span(0..1),
                Node::new(3..5, vec![
                    (OpAdd::from_span(3..4), Ident::from_span(4..5)),
                ]).into(),
            ),
        ));

        let root = parser.parse::<SeparatedList>()?;
        assert_eq!(parser.remaining(), "");
        assert_eq!(root, Some(
            SeparatedList(
                Ident::from_span(6..7),
                Node::new(7..9, vec![
                    (OpAdd::from_span(7..8), Ident::from_span(8..9)),
                ]).into(),
            ),
        ));
    
        Ok(())
    }

    #[test]
    fn parse_separated_list_panic() -> Result<(), GrammarError> {
        let mut parser = Parser::<T>::new("a + b + +e")?;
        let root = parser.parse::<SeparatedList>()?;
        assert_eq!(parser.remaining(), "+e");
        assert_eq!(root, Some(
            SeparatedList(
                Ident::from_span(0..1),
                Node::new(2..5, vec![
                    (OpAdd::from_span(2..3), Ident::from_span(4..5)),
                ]).into(),
            ),
        ));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(8..9, ErrorKind::Expecting(first_set!(T{Ident:*}))),
        ]);

        Ok(())
    }

    // TODO - star tests

}
