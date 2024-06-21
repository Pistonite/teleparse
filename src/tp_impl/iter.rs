use std::borrow::Cow;
use std::marker::PhantomData;

use crate::syntax::{self, ErrorKind, FirstSet, Metadata, MetadataBuilder, Result as SynResult};
use crate::{Parser, Pos, Produce, Production, Span, ToSpan};

use super::option::OptionProd;
use super::Node;

// OneOrMore<T> => T Option<OneOrMore<T>>
#[doc(hidden)]
pub struct OneOrMore<T: Production>(PhantomData<T>);
impl<T: Production> Production for OneOrMore<T> {
    type L = T::L;
    #[inline]
    fn debug() -> Cow<'static, str> {
        Cow::Owned(format!("( {} )+", T::debug()))
    }

    fn register(meta: &mut MetadataBuilder<Self::L>) {
        crate::register_sequence!(meta, T, OptionProd<OneOrMore<T>>)
    }
}


#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Plus<V: FromIterator<T>, T: Produce>(Node<V>, PhantomData<T>);

impl<V: FromIterator<T> + std::fmt::Debug, T: Produce> std::fmt::Debug for Plus<V, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<V: FromIterator<T>, T: Produce> Produce for Plus<V, T> {
    type Prod = OneOrMore<T::Prod>;
    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        let (first_elem, mut errors) = match T::produce(parser, meta) {
            SynResult::Success(t) => {
                (t, Vec::new())
            }
            SynResult::Recovered(t, e) => {
                (t, e)
            }
            SynResult::Panic(e) => {
                return SynResult::Panic(e);
            }
        };
        let mut hi = first_elem.hi();
        let produce_iter = ProduceIter::new(
            parser, meta, 
            meta.first.get(&Self::prod_id()), &mut errors, &mut hi);
        let lo = first_elem.lo();
        let v: V = std::iter::once(first_elem)
            .chain(produce_iter)
            .collect();
        (Node::new(lo..hi, v).into(), errors).into()
    }

}

#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Star<V: FromIterator<T> + Default, T: Produce>(Node<V>, PhantomData<T>);

impl<V: FromIterator<T> + Default + std::fmt::Debug, T: Produce> std::fmt::Debug for Star<V, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<V: FromIterator<T>+ Default, T: Produce> Produce for Star<V, T> {
    type Prod = OptionProd<OneOrMore<T::Prod>>;
    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        <super::option::Optional<Plus<V, T>>>::produce(parser, meta)
            .map(|n| match n.0.value {
                Some(t) => Node::new(t.span(), t.into_inner()).into(),
                None => Node::new(n.span(), V::default()).into(),
            })
    }

}

#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Loop<T: Produce>(Node<Vec<T>>);

impl<T: Produce + std::fmt::Debug> std::fmt::Debug for Loop<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Produce> Produce for Loop<T> {
    type Prod = OneOrMore<T::Prod>;
    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        let mut errors = Vec::new();
        let mut output = Vec::new();
        let first = meta.first.get(&Self::prod_id());

        loop {
            let mut token = parser.peek_token_src();
            if token.is_none() {
                break;
            }
            if !first.contains(token) {
                let span = parser.current_span();
                let skip_lo = span.lo;
                // we need to keep track of hi instead of using the lo
                // of a valid token, because there could be skipped characters between.
                let mut skip_hi = span.hi;
                parser.consume_token();
                token = parser.peek_token_src();
                while token.is_some() && !first.contains(token) {
                    skip_hi = parser.current_span().hi;
                    parser.consume_token();
                    token = parser.peek_token_src();
                }
                errors.push(syntax::Error::new(
                    skip_lo..skip_hi,
                    ErrorKind::UnexpectedTokens,
                ));
                if token.is_none() {
                    break;
                }
            }
            let lo_before = parser.current_span().lo;
            match T::produce(parser, meta) {
                SynResult::Success(t) => {
                    output.push(t);
                }
                SynResult::Recovered(t, e) => {
                    output.push(t);
                    errors.extend(e);
                }
                SynResult::Panic(e) => {
                    errors.extend(e);
                }
            }
            let span = parser.current_span();
            if lo_before == span.lo {
                errors.push(syntax::Error::new(
                    span,
                    ErrorKind::UnexpectedNoAdvanceInLoop,
                ));
                break;
            }
        }
        let span = if let (Some(first), Some(last)) = (output.first(), output.last()) {
            Span::new(first.lo(), last.hi())
        } else {
            parser.current_span_empty()
        };
        (Node::new(span, output).into(), errors).into()
    }

}

struct ProduceIter<'a, 'b, 'c, 'd, 's, T: Produce> {
    parser: &'a mut Parser<'s, <T::Prod as Production>::L>,
    meta: &'b Metadata<<T::Prod as Production>::L>,
    first: &'b FirstSet<<T::Prod as Production>::L>,
    errors: &'c mut Vec<syntax::Error<<T::Prod as Production>::L>>,
    hi: &'d mut Pos,
}
impl<'a, 'b, 'c, 'd, 's, T: Produce> ProduceIter<'a, 'b, 'c, 'd, 's, T> {
    pub fn new(
        parser: &'a mut Parser<'s, <T::Prod as Production>::L>,
        meta: &'b Metadata<<T::Prod as Production>::L>,
        first: &'b FirstSet<<T::Prod as Production>::L>,
        errors: &'c mut Vec<syntax::Error<<T::Prod as Production>::L>>,
        hi: &'d mut Pos,
    ) -> Self {
        Self {
            parser,
            meta,
            first,
            errors,
            hi
        }
    }
}

impl<T: Produce> Iterator for ProduceIter<'_, '_, '_, '_, '_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.parser.peek_token_src()?;
        if !self.first.contains(Some(token)) {
            return None;
        }
        match T::produce(self.parser, self.meta) {
            SynResult::Success(t) => {
                *self.hi = t.hi();
                Some(t)
            }
            SynResult::Recovered(t, e) => {
                self.errors.extend(e);
                *self.hi = t.hi();
                Some(t)
            }
            SynResult::Panic(e) => {
                self.errors.extend(e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::syntax::ErrorKind;
    use crate::{syntax, Parser, GrammarError};
    
    use crate::test::prelude::*;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd};
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct IdentList(tp::Nev<Ident>);
    
    #[test]
    fn parse_single_item() -> Result<(), GrammarError> {
        let t = IdentList::parse("a")?.unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..1 => [token Ident(0..1)]");
        assert_eq!(t, IdentList(Node::new(0..1, vec![
            Ident::from_span(0..1)
        ]).into()));

        Ok(())
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
            "( (Ident)? )+".to_string(),
            "(Ident)?".to_string(),
            "( (Ident)? )*".to_string(),
            "Ident".to_string()
        ));
    }
    
    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq, Clone)]
    struct SameList(tp::Nev<OpAdd>, tp::Nev<OpAdd>);
    
    #[test]
    fn two_list_of_same_not_ll1() {
        assert_not_ll1!(SameList, GrammarError::FirstFollowConflict(
            "( + )*".to_string(),
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
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct IdentListStar(tp::Star<Vec<Ident>, Ident>);
    
    #[test]
    fn parse_star() {
        let t = IdentListStar::parse("").unwrap().unwrap();
        assert_eq!(t, IdentListStar(Node::new(0..0, vec![]).into()));
    
        let t = IdentListStar::parse("a b c").unwrap().unwrap();
        assert_eq!(t, IdentListStar(Node::new(0..5, vec![
            Ident::from_span(0..1),
            Ident::from_span(2..3),
            Ident::from_span(4..5),
        ]).into()));
    }
    
    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq, Clone)]
    struct ConsecutiveStar(tp::Star<Vec<Ident>, Ident>, tp::Vec<Ident>);
    
    #[test]
    fn consecutive_star_not_ll1() {
        assert_not_ll1!(ConsecutiveStar, GrammarError::FirstFollowSeqConflict(
            "ConsecutiveStar".to_string(),
            "( Ident )*".to_string(),
            "( Ident )*".to_string(),
            "Ident".to_string()
        ));
    }
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct LoopRoot(tp::Loop<Ident>);
    
    #[test]
    fn parse_loop_empty() {
        let t = LoopRoot::parse("").unwrap().unwrap();
        assert_eq!(t, LoopRoot(Node::new(0..0, vec![]).into()));
    }

    #[test]
    fn parse_loop_empty_all_invalid() {
        let t = LoopRoot::parse("+++").unwrap().unwrap();
        assert_eq!(t, LoopRoot(Node::new(3..3, vec![]).into()));
    }
    
    #[test]
    fn parse_loop() {
        let t = LoopRoot::parse("a b c").unwrap().unwrap();
        assert_eq!(t, LoopRoot(Node::new(0..5, vec![
            Ident::from_span(0..1),
            Ident::from_span(2..3),
            Ident::from_span(4..5),
        ]).into()));
    }
    
    #[test]
    fn parse_loop_recover() -> Result<(), GrammarError> {
        let mut parser = Parser::<T>::new("+a+ b++ c")?;
        let t = parser.parse::<LoopRoot>()?.unwrap();
        assert_eq!(t, LoopRoot(Node::new(1..9, vec![
            Ident::from_span(1..2),
            Ident::from_span(4..5),
            Ident::from_span(8..9),
        ]).into()));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(0..1, ErrorKind::UnexpectedTokens),
            syntax::Error::new(2..3, ErrorKind::UnexpectedTokens),
            syntax::Error::new(5..7, ErrorKind::UnexpectedTokens),
        ]);
    
        Ok(())
    }
    
    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq, Clone)]
    struct LoopOption(tp::Loop<tp::Option<Ident>>);
    
    #[test]
    fn loop_option_not_ll1() {
        assert_not_ll1!(LoopOption, GrammarError::FirstFollowSeqConflict(
            "( (Ident)? )+".to_string(),
            "(Ident)?".to_string(),
            "( (Ident)? )*".to_string(),
            "Ident".to_string()
        ));
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct LoopRecover(tp::Loop<tp::Recover<Ident, tp::Option<OpAdd>>>);

    #[test]
    fn loop_will_not_stuck() -> Result<(), GrammarError> {
        let mut parser = Parser::<T>::new("((((")?;
        //---------------------------------^^^^ UnexpectedTokens
        let t = parser.parse::<LoopRecover>()?.unwrap();
        assert_eq!(t, LoopRecover(Node::new(4..4, vec![]).into()));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(0..4, ErrorKind::UnexpectedTokens),
        ]);
    
        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq, Clone)]
    struct LoopRecover2(tp::Loop<tp::Recover<tp::Option<Ident>, tp::Option<OpAdd>>>);

    #[test]
    fn loop_recover_option_not_ll1() {
        assert_not_ll1!(LoopRecover2, GrammarError::FirstFollowSeqConflict(
            "( (Ident)? (+)? )+".to_string(),
            "(Ident)? (+)?".to_string(),
            "( (Ident)? (+)? )*".to_string(),
            "\"+\", Ident".to_string()
        ));
    }

}
