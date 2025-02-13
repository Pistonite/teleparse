use std::ops::{Deref, DerefMut};

use crate::syntax::{self, Metadata, Result as SynResult};
use crate::{Parser, Produce, Production, ToSpan};

use super::iter::OneOrMore;
use super::option::OptionProd;
use super::Node;

#[derive(ToSpan, Debug, Clone, PartialEq)]
pub struct Split<T: Produce, P: Produce>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    pub elems: Node<Vec<T>>,
    pub puncts: Vec<P>,
}

impl<T: Produce, P: Produce> Deref for Split<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.elems
    }
}

impl<T: Produce, P: Produce> DerefMut for Split<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elems
    }
}

impl<T: Produce, P: Produce> IntoIterator for Split<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_inner().into_iter()
    }
}

impl<'a, T: Produce, P: Produce> IntoIterator for &'a Split<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter()
    }
}

impl<'a, T: Produce, P: Produce> IntoIterator for &'a mut Split<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter_mut()
    }
}

impl<T: Produce, P: Produce> Produce for Split<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    // Split<T> => T (P T)*
    type Prod = (T::Prod, OptionProd<OneOrMore<(P::Prod, T::Prod)>>);
    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        let (lo, mut elems, mut errors) = match T::produce(parser, meta) {
            SynResult::Success(t) => (t.lo(), vec![t], Vec::new()),
            SynResult::Recovered(t, e) => (t.lo(), vec![t], e),
            SynResult::Panic(e) => {
                return SynResult::Panic(e);
            }
        };
        let mut puncts = Vec::new();
        let elem_first = meta.first.get(&T::prod_id());
        let punct_first = meta.first.get(&P::prod_id());
        let follow = meta.follow.get(&Self::prod_id());
        'outer: loop {
            // expecting (P)?
            let token = parser.peek_token_src();
            if token.is_none() {
                break;
            }
            if !punct_first.contains(token) {
                break;
            }
            // expecting P
            let p = match P::produce(parser, meta) {
                SynResult::Success(p) => p,
                SynResult::Recovered(p, e) => {
                    errors.extend(e);
                    p
                }
                SynResult::Panic(e) => {
                    errors.extend(e);
                    break;
                }
            };
            // expecting T
            let mut panic: Option<syntax::Error<<Self::Prod as Production>::L>> = None;
            loop {
                let mut token = parser.peek_token_src();
                if elem_first.contains(token) || elem_first.contains_epsilon() {
                    match T::produce(parser, meta) {
                        SynResult::Success(t) => {
                            elems.push(t);
                            puncts.push(p);
                            if let Some(e) = panic {
                                errors.push(e);
                            }
                            continue 'outer;
                        }
                        SynResult::Recovered(t, e) => {
                            elems.push(t);
                            puncts.push(p);
                            errors.extend(e);
                            if let Some(e) = panic {
                                errors.push(e);
                            }
                            continue 'outer;
                        }
                        SynResult::Panic(e) => {
                            if let Some(e) = e.into_iter().next_back() {
                                if let Some(p) = &mut panic {
                                    p.span.hi = e.span.hi;
                                } else {
                                    panic = Some(e);
                                }
                            }
                        }
                    }
                    // recovery
                    token = parser.peek_token_src();
                } else if let Some(p) = &mut panic {
                    p.span.hi = parser.current_span().hi;
                } else {
                    panic = Some(parser.expecting(elem_first.clone()));
                }

                if punct_first.contains(token) {
                    if let Some(e) = panic {
                        errors.push(e);
                    }
                    continue 'outer;
                }
                if token.is_none() || follow.contains(token) {
                    if let Some(e) = panic {
                        errors.push(e);
                    }
                    break 'outer;
                }
                parser.consume_token();
            }
        }
        let hi = elems.last().map_or_else(|| lo, |t| t.hi());
        let s = Self {
            elems: Node::new(lo..hi, elems),
            puncts,
        };
        (s, errors).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::syntax::{self, ErrorKind};

    use crate::test::prelude::*;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd, ParenClose, ParenOpen};
    use crate::{GrammarError, Parser};

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct SplitTest(tp::Split<Ident, OpAdd>);

    #[test]
    fn parse_empty() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(")?;
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(result, None);
        assert_eq!(
            parser.info().errors,
            vec![syntax::Error::new(
                0..1,
                ErrorKind::Expecting(first_set!(T {Ident:*}))
            )]
        );

        Ok(())
    }

    #[test]
    fn parse_one() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a")?;
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(0..1, vec![Ident::from_span(0..1)]),
                puncts: vec![]
            }))
        );

        Ok(())
    }

    #[test]
    fn parse_two() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+b")?;
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(0..3, vec![Ident::from_span(0..1), Ident::from_span(2..3)]),
                puncts: vec![OpAdd::from_span(1..2)]
            }))
        );
        // can use split as iterator for elems
        let result = result.unwrap().0;
        assert_eq!(result.len(), 2);
        let mut iter = result.iter();
        assert_eq!(iter.next(), Some(&Ident::from_span(0..1)));
        assert_eq!(iter.next(), Some(&Ident::from_span(2..3)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        // iterator tests
        for _ in &result {}
        let mut result = result;
        for _ in &mut result {}
        let result = result;
        for _ in result {}

        Ok(())
    }

    #[test]
    fn parse_many() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+b+c+d")?;
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(
                    0..7,
                    vec![
                        Ident::from_span(0..1),
                        Ident::from_span(2..3),
                        Ident::from_span(4..5),
                        Ident::from_span(6..7)
                    ]
                ),
                puncts: vec![
                    OpAdd::from_span(1..2),
                    OpAdd::from_span(3..4),
                    OpAdd::from_span(5..6)
                ]
            }))
        );
        assert!(parser.info().errors.is_empty());

        Ok(())
    }

    #[test]
    fn parse_stop_expecting_split() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a + b c + d")?;
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(0..5, vec![Ident::from_span(0..1), Ident::from_span(4..5),]),
                puncts: vec![OpAdd::from_span(2..3),]
            }))
        );
        assert_eq!(parser.remaining(), "c + d");
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(
                    6..11,
                    vec![Ident::from_span(6..7), Ident::from_span(10..11),]
                ),
                puncts: vec![OpAdd::from_span(8..9),]
            }))
        );

        Ok(())
    }

    #[test]
    fn parse_recover_expecting_item() -> Result<(), GrammarError> {
        // skips ( and parses a + c
        let mut parser = Parser::new("a+(c)+")?;
        //------------------------------^ expecting Ident
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(0..4, vec![Ident::from_span(0..1), Ident::from_span(3..4),]),
                puncts: vec![OpAdd::from_span(1..2),]
            }))
        );
        assert_eq!(parser.remaining(), ")+");
        assert_eq!(
            parser.info().errors,
            vec![syntax::Error::new(
                2..3,
                ErrorKind::Expecting(first_set!(T {Ident:*}))
            )]
        );

        Ok(())
    }

    #[test]
    fn parse_recover_skip_sees_separator() -> Result<(), GrammarError> {
        // skips (, sees + and discards the previous, parses a + c
        let mut parser = Parser::new("a+(+c")?;
        //------------------------------^^ expecting Ident
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(0..5, vec![Ident::from_span(0..1), Ident::from_span(4..5),]),
                puncts: vec![OpAdd::from_span(3..4),]
            }))
        );
        assert_eq!(parser.remaining(), "");
        assert_eq!(
            parser.info().errors,
            vec![syntax::Error::new(
                2..4,
                ErrorKind::Expecting(first_set!(T {Ident:*}))
            )]
        );

        Ok(())
    }

    #[test]
    fn parse_recover_expecting_item_skip_multiple() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+((+c")?;
        //------------------------------^^^ expecting Ident
        let result = parser.parse::<SplitTest>()?;
        assert_eq!(
            result,
            Some(SplitTest(tp::Split {
                elems: Node::new(0..6, vec![Ident::from_span(0..1), Ident::from_span(5..6),]),
                puncts: vec![OpAdd::from_span(4..5),]
            }))
        );
        assert_eq!(parser.remaining(), "");
        assert_eq!(
            parser.info().errors,
            vec![syntax::Error::new(
                2..5,
                ErrorKind::Expecting(first_set!(T {Ident:*}))
            )]
        );

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct SplitTestFollow(tp::Split<Ident, OpAdd>, ParenClose);

    #[test]
    fn parse_recover_follow() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+c+()")?;
        //--------------------------------^^ expecting Ident
        let result = parser.parse::<SplitTestFollow>()?;
        assert_eq!(
            result,
            Some(SplitTestFollow(
                tp::Split {
                    elems: Node::new(0..3, vec![Ident::from_span(0..1), Ident::from_span(2..3),]),
                    puncts: vec![OpAdd::from_span(1..2),]
                },
                ParenClose::from_span(5..6)
            ))
        );
        assert_eq!(parser.remaining(), "");
        assert_eq!(
            parser.info().errors,
            vec![syntax::Error::new(
                4..6,
                ErrorKind::Expecting(first_set!(T {Ident:*}))
            )]
        );

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct SplitOfOption(tp::Split<tp::Option<Ident>, OpAdd>);

    #[test]
    fn parse_split_option() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+++c")?;
        let result = parser.parse::<SplitOfOption>()?;
        assert_eq!(
            result,
            Some(SplitOfOption(tp::Split {
                elems: Node::new(
                    0..5,
                    vec![
                        Node::new(0..1, Some(Ident::from_span(0..1))).into(),
                        Node::new(2..2, None).into(),
                        Node::new(3..3, None).into(),
                        Node::new(4..5, Some(Ident::from_span(4..5))).into(),
                    ]
                ),
                puncts: vec![
                    OpAdd::from_span(1..2),
                    OpAdd::from_span(2..3),
                    OpAdd::from_span(3..4),
                ]
            }))
        );
        assert_eq!(parser.remaining(), "");

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct SplitSame(tp::Split<Ident, Ident>);

    #[test]
    fn parse_split_same() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a b c d e")?;
        let result = parser.parse::<SplitSame>()?;
        assert_eq!(
            result,
            Some(SplitSame(tp::Split {
                elems: Node::new(
                    0..9,
                    vec![
                        Ident::from_span(0..1),
                        Ident::from_span(4..5),
                        Ident::from_span(8..9),
                    ]
                ),
                puncts: vec![Ident::from_span(2..3), Ident::from_span(6..7),]
            }))
        );
        assert_eq!(parser.remaining(), "");

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq)]
    struct SplitOptionSame(tp::Split<tp::Option<Ident>, Ident>);

    #[test]
    fn split_option_same_conflict() {
        assert_not_ll1!(
            SplitOptionSame,
            GrammarError::FirstFollowSeqConflict(
                "(Ident)? ( Ident (Ident)? )*".to_string(),
                "(Ident)?".to_string(),
                "( Ident (Ident)? )*".to_string(),
                "Ident".to_string()
            )
        );
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq)]
    struct SplitOptionSepSame(tp::Split<Ident, tp::Option<Ident>>);

    #[test]
    fn split_option_same_conflict_sep() {
        assert_not_ll1!(
            SplitOptionSepSame,
            GrammarError::FirstFollowSeqConflict(
                "(Ident)? Ident".to_string(),
                "(Ident)?".to_string(),
                "Ident".to_string(),
                "Ident".to_string()
            )
        );
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct Split2(tp::Split<(ParenOpen, Ident, ParenClose), OpAdd>);

    #[test]
    fn parse_partial_item_recover_autofill() -> Result<(), GrammarError> {
        // recover is provided by terminal, not split
        let mut parser = Parser::new("(a)+(b+(c)")?;
        //----------------------------------^ expecting )
        let result = parser.parse::<Split2>()?;
        assert_eq!(
            result,
            Some(Split2(tp::Split {
                elems: Node::new(
                    0..10,
                    vec![
                        (
                            ParenOpen::from_span(0..1),
                            Ident::from_span(1..2),
                            ParenClose::from_span(2..3)
                        ),
                        (
                            ParenOpen::from_span(4..5),
                            Ident::from_span(5..6),
                            ParenClose::from_span(6..6)
                        ),
                        (
                            ParenOpen::from_span(7..8),
                            Ident::from_span(8..9),
                            ParenClose::from_span(9..10)
                        ),
                    ]
                ),
                puncts: vec![OpAdd::from_span(3..4), OpAdd::from_span(6..7),]
            }))
        );
        assert_eq!(parser.remaining(), "");
        assert_eq!(
            parser.info().errors,
            vec![syntax::Error::new(
                6..7,
                ErrorKind::Expecting(first_set!(T { Paren: ")" }))
            )]
        );

        Ok(())
    }

    #[test]
    fn parse_partial_item_recover_skip() -> Result<(), GrammarError> {
        // recover is provided by split
        let mut parser = Parser::new("(a)+(+(c)")?;
        //---------------------------------^ expecting Ident
        let result = parser.parse::<Split2>()?;
        assert_eq!(
            result,
            Some(Split2(tp::Split {
                elems: Node::new(
                    0..9,
                    vec![
                        (
                            ParenOpen::from_span(0..1),
                            Ident::from_span(1..2),
                            ParenClose::from_span(2..3)
                        ),
                        (
                            ParenOpen::from_span(6..7),
                            Ident::from_span(7..8),
                            ParenClose::from_span(8..9)
                        ),
                    ]
                ),
                puncts: vec![OpAdd::from_span(5..6),]
            }))
        );
        assert_eq!(parser.remaining(), "");
        assert_eq!(
            parser.info().errors,
            vec![syntax::Error::new(
                5..6,
                ErrorKind::Expecting(first_set!(T {Ident:*}))
            )]
        );

        Ok(())
    }

    // this is not LL(1) because something like a + b
    // can be either Some(a) Some(+) Some(b)
    // or something like Some(a) None None Some(+) Some(b)
    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq)]
    struct SplitBothOption(tp::Split<tp::Option<Ident>, tp::Option<OpAdd>>);

    #[test]
    fn split_both_option_conflict() {
        assert_not_ll1!(
            SplitBothOption,
            GrammarError::FirstFollowSeqConflict(
                "(Ident)? ( (+)? (Ident)? )*".to_string(),
                "(Ident)?".to_string(),
                "( (+)? (Ident)? )*".to_string(),
                "Ident".to_string()
            )
        );
    }
}
