
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::marker::PhantomData;

use crate::syntax::{First, FirstBuilder, FirstRel, Follow, FollowBuilder, FollowRel, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, ParseTree, Parser, Span, ToSpan};

use super::{Node, OptionAST};

pub struct OneOrMore<T: AbstractSyntaxTree>(Vec<T>);

impl<T: AbstractSyntaxTree> ToSpan for OneOrMore<T> {
    fn span(&self) -> Span {
        let lo = self.0.first().map(|t| t.span().lo).unwrap_or_default();
        let hi = self.0.last().map(|t| t.span().hi).unwrap_or_default();
        Span::new(lo, hi)
    }
}

impl<T: AbstractSyntaxTree> AbstractSyntaxTree for OneOrMore<T> {
    type L=T::L;
    fn debug() -> Cow<'static, str> {
        Cow::Owned(format!("OneOrMore<{}>", T::debug()))
    }
    fn build_first(builder: &mut FirstBuilder<Self::L>) {
        // OneOrMore<T> => T Option<OneOrMore<T>>
        let t = Self::type_id();
        if builder.visit(t, &Self::debug()) {
            // recursive build
            T::build_first(builder);
            OptionAST::<OneOrMore<T>>::build_first(builder);
            let inner = T::type_id();
            builder.build_sequence(t, &[inner, OptionAST::<OneOrMore<T>>::type_id()]);
        }
    }
    fn check_left_recursive(
        seen: &mut BTreeSet<TypeId>,
        stack: &mut Vec<String>,
        set: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
    ) -> Result<(), GrammarError> {
        let t = Self::type_id();
        if !seen.insert(t) {
            return Ok(());
        }
        if !set.insert(t) {
            return Err(GrammarError::left_recursion(&stack, &Self::debug()));
        }
        stack.push(Self::debug().into_owned());
        let result = T::check_left_recursive(seen, stack, set, first);
        stack.pop();
        set.remove(&t);
        result
    }
    fn check_first_conflict(
        seen: &mut BTreeSet<TypeId>, 
        first: &First<Self::L>
    ) -> Result<(), GrammarError> {
        let t = Self::type_id();
        if !seen.insert(t) {
            return Ok(());
        }
        let first_set = first.get(&t);
        if first_set.contains_epsilon() {
            return Err(GrammarError::FirstFollowSeqConflict(
                Self::debug().into_owned(),
                T::debug().into_owned(),
                OptionAST::<OneOrMore<T>>::debug().into_owned(),
                "<epsilon>".to_string(),
            ))
        }

        T::check_first_conflict(seen, first)?;

        Ok(())
    }
    fn build_follow( builder: &mut FollowBuilder<Self::L>) {
        let t = Self::type_id();
        if builder.visit(t) {
            // recursive build
            T::build_follow(builder);
            OptionAST::<OneOrMore<T>>::build_follow(builder);
            let inner = T::type_id();
            builder.build_sequence(t, &[inner, OptionAST::<OneOrMore<T>>::type_id()]);
        }
    }
    fn check_first_follow_conflict(
        seen: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
        follow: &Follow<Self::L>,
    ) -> Result<(), GrammarError> {
        if !seen.insert(Self::type_id()) {
            return Ok(());
        }
        Self::check_self_first_follow_conflict(first, follow)?;
        T::check_first_follow_conflict(seen, first, follow)?;
        OptionAST::<OneOrMore<T>>::check_first_follow_conflict(seen, first, follow)
    }
    fn build_jump(
        seen: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
        jump: &mut Jump<Self::L>
    ) {
        if seen.insert(Self::type_id()) {
            T::build_jump(seen, first, jump);
        }
    }
    fn parse_ast<'s>(
        parser: &mut Parser<'s, Self::L>,
        meta: &Metadata<Self::L>,
    ) -> SynResult<Self, Self::L> {
        let (mut output, mut errors) = match T::parse_ast(parser, meta) {
            SynResult::Success(t) => {
                (vec![t], Vec::new())
            }
            SynResult::Recovered(t, e) => {
                (vec![t], e)
            }
            SynResult::Panic(e) => {
                return SynResult::Panic(e);
            }
        };
    let t_type = T::type_id();
    let first = meta.first.get(&t_type);
    loop {
        let token = parser.peek_token_src();
        if token.is_none() {
            break;
        }
        if !first.contains(token) {
            break;
        }
        match T::parse_ast(parser, meta) {
            SynResult::Success(t) => {
                output.push(t);
            }
            SynResult::Recovered(t, e) => {
                output.push(t);
                errors.extend(e);
            }
            SynResult::Panic(e) => {
                // delay to parent to panic
                errors.extend(e);
                return SynResult::Recovered(Self(output), errors);
            }
        }
    }
    if errors.is_empty() {
        SynResult::Success(Self(output))
    } else {
        SynResult::Recovered(Self(output), errors)
    }
    }
}

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
    #[derive(Debug, PartialEq)]
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
    fn parse_two_items() {
        let mut parser = Parser::<T>::new("a b").unwrap();
        let t = parser.parse_one::<IdentList>().unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..3 => [token Ident(0..1), token Ident(2..3)]");
        assert_eq!(t, IdentList(Node::new(0..3, vec![
            Ident::from_span(0..1),
            Ident::from_span(2..3),
        ]).into()));
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
        let roots = parser.parse_all::<IdentList>()?;

        let mut first = FirstSet::new();
        first.insert(T::Ident, None);
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(5..6, ErrorKind::Expecting(first))
        ]);

        assert_eq!(roots, vec![
            IdentList(Node::new(0..5, vec![
                Ident::from_span(0..1),
                Ident::from_span(2..3),
                Ident::from_span(4..5),
            ]).into()),
            IdentList(Node::new(6..7, vec![
                Ident::from_span(6..7),
            ]).into()),
        ]);
        

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
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
    #[derive(Debug, PartialEq)]
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
    #[derive(Debug, PartialEq)]
    struct SameList(tp::Nev<OpAdd>, tp::Nev<OpAdd>);

    #[test]
    fn two_list_of_same_not_ll1() {
        assert_not_ll1!(SameList, GrammarError::FirstFollowConflict(
            "Option<OneOrMore<OpAdd>>".to_string(),
            "\"+\"".to_string(),
        ));
    }

    // todo: ident ( +ident)+ and its panic behavior
}
