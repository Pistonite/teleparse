use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::syntax::{self, Metadata, MetadataBuilder, Result as SynResult};
use crate::{Parser, Produce, Production, Span, ToSpan};

use super::option::OptionProd;

// Punct<T, P> => T (P (Punct<T, P>)?)?
#[doc(hidden)]
pub struct PunctProd<T: Production, P: Production<L = T::L>>(PhantomData<(T, P)>);
impl<T: Production, P: Production<L = T::L>> Production for PunctProd<T, P> {
    type L = T::L;
    #[inline]
    fn debug() -> Cow<'static, str> {
        Cow::Owned(format!("( {} )[{}]+", T::debug(), P::debug()))
    }

    fn register(meta: &mut MetadataBuilder<Self::L>) {
        crate::register_sequence!(meta, T, OptionProd<(P, OptionProd<Self>)>)
    }
}

#[derive(ToSpan, Debug, Clone, PartialEq)]
pub struct Punct<T: Produce, P: Produce>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    pub span: Span,
    pub elems: Vec<T>,
    pub puncts: Vec<P>,
}

impl<T: Produce, P: Produce> Deref for Punct<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.elems
    }
}

impl<T: Produce, P: Produce> DerefMut for Punct<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elems
    }
}

impl<T: Produce, P: Produce> IntoIterator for Punct<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl<'a, T: Produce, P: Produce> IntoIterator for &'a Punct<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter()
    }
}

impl<'a, T: Produce, P: Produce> IntoIterator for &'a mut Punct<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter_mut()
    }
}

impl<T: Produce, P: Produce> Produce for Punct<T, P>
where
    P::Prod: Production<L = <T::Prod as Production>::L>,
{
    type Prod = PunctProd<T::Prod, P::Prod>;
    // This does not use Split because of different recovery logic
    // Split cannot end on separator, so it will try to find an item and
    // "commit" a (P T) pair.
    // Punct will always commit the P first, and try to parse T if it can, or stop
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
            // expecting (P (Punct<T, P>)?)?
            let token = parser.peek_token_src();
            if token.is_none() {
                break;
            }
            if !punct_first.contains(token) {
                break;
            }
            // expecting P (Punct<T, P>)?
            match P::produce(parser, meta) {
                SynResult::Success(p) => {
                    puncts.push(p);
                }
                SynResult::Recovered(p, e) => {
                    errors.extend(e);
                    puncts.push(p);
                }
                SynResult::Panic(e) => {
                    errors.extend(e);
                    break;
                }
            };
            let mut panic: Option<syntax::Error<<Self::Prod as Production>::L>> = None;
            loop {
                // expecting (Punct<T, P>)?
                let token = parser.peek_token_src();
                if token.is_none() {
                    break;
                }
                if !elem_first.contains(token) {
                    break;
                }
                // expecting Punct<T, P> => T (P (Punct<T, P>)?)?
                match T::produce(parser, meta) {
                    SynResult::Success(t) => {
                        elems.push(t);
                        continue 'outer;
                    }
                    SynResult::Recovered(t, e) => {
                        elems.push(t);
                        errors.extend(e);
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
                let token = parser.peek_token_src();
                if punct_first.contains(token) {
                    puncts.pop();
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
        let hi_1 = elems.last().map_or_else(|| lo, |t| t.hi());
        let hi_2 = puncts.last().map_or_else(|| lo, |t| t.hi());
        let hi = hi_1.max(hi_2);
        let s = Self {
            span: Span::new(lo, hi),
            elems,
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
    struct PunctTest(tp::Punct<Ident, OpAdd>);

    #[test]
    fn parse_empty() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(")?;
        let result = parser.parse::<PunctTest>()?;
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
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(0..1),
                elems: vec![Ident::from_span(0..1)],
                puncts: vec![]
            }))
        );

        Ok(())
    }

    #[test]
    fn parse_one_trailing() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+")?;
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(0..2),
                elems: vec![Ident::from_span(0..1)],
                puncts: vec![OpAdd::from_span(1..2)]
            }))
        );

        Ok(())
    }

    #[test]
    fn parse_two() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+b")?;
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(0..3),
                elems: vec![Ident::from_span(0..1), Ident::from_span(2..3)],
                puncts: vec![OpAdd::from_span(1..2)]
            }))
        );
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
    fn parse_two_trailing() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+b+")?;
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(0..4),
                elems: vec![Ident::from_span(0..1), Ident::from_span(2..3)],
                puncts: vec![OpAdd::from_span(1..2), OpAdd::from_span(3..4),]
            }))
        );
        let result = result.unwrap().0;
        assert_eq!(result.len(), 2);
        let mut iter = result.iter();
        assert_eq!(iter.next(), Some(&Ident::from_span(0..1)));
        assert_eq!(iter.next(), Some(&Ident::from_span(2..3)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        Ok(())
    }

    #[test]
    fn parse_many() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+b+c+d")?;
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(0..7),
                elems: vec![
                    Ident::from_span(0..1),
                    Ident::from_span(2..3),
                    Ident::from_span(4..5),
                    Ident::from_span(6..7)
                ],
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
    fn parse_many_trailing() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+b+c+d+")?;
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(0..8),
                elems: vec![
                    Ident::from_span(0..1),
                    Ident::from_span(2..3),
                    Ident::from_span(4..5),
                    Ident::from_span(6..7)
                ],
                puncts: vec![
                    OpAdd::from_span(1..2),
                    OpAdd::from_span(3..4),
                    OpAdd::from_span(5..6),
                    OpAdd::from_span(7..8)
                ]
            }))
        );
        assert!(parser.info().errors.is_empty());

        Ok(())
    }

    #[test]
    fn parse_stop_expecting_split() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a + b c + d")?;
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(0..5),
                elems: vec![Ident::from_span(0..1), Ident::from_span(4..5),],
                puncts: vec![OpAdd::from_span(2..3),]
            }))
        );
        assert_eq!(parser.remaining(), "c + d");
        let result = parser.parse::<PunctTest>()?;
        assert_eq!(
            result,
            Some(PunctTest(tp::Punct {
                span: Span::from(6..11),
                elems: vec![Ident::from_span(6..7), Ident::from_span(10..11),],
                puncts: vec![OpAdd::from_span(8..9),]
            }))
        );

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct ParenPunct(ParenOpen, tp::Punct<Ident, OpAdd>);

    #[test]
    fn parse_stop_expecting_item() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(a + b + (c + d")?;
        let result = parser.parse::<ParenPunct>()?;
        assert_eq!(
            result,
            Some(ParenPunct(
                ParenOpen::from_span(0..1),
                tp::Punct {
                    span: Span::from(1..8),
                    elems: vec![Ident::from_span(1..2), Ident::from_span(5..6),],
                    puncts: vec![OpAdd::from_span(3..4), OpAdd::from_span(7..8),]
                }
            ))
        );
        assert_eq!(parser.remaining(), "(c + d");
        let result = parser.parse::<ParenPunct>()?;
        assert_eq!(
            result,
            Some(ParenPunct(
                ParenOpen::from_span(9..10),
                tp::Punct {
                    span: Span::from(10..15),
                    elems: vec![Ident::from_span(10..11), Ident::from_span(14..15),],
                    puncts: vec![OpAdd::from_span(12..13),]
                }
            ))
        );

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct Punct2(tp::Punct<(ParenOpen, Ident, ParenClose), OpAdd>);

    #[test]
    fn parse_partial_item_recover_autofill() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(a)+(b+(c)")?;
        //----------------------------------^ expecting )
        let result = parser.parse::<Punct2>()?;
        assert_eq!(
            result,
            Some(Punct2(tp::Punct {
                span: Span::from(0..10),
                elems: vec![
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
                ],
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
    fn parse_partial_item_recover_autofill_trailing() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(a)+(b+(c)+")?;
        //----------------------------------^ expecting )
        let result = parser.parse::<Punct2>()?;
        assert_eq!(
            result,
            Some(Punct2(tp::Punct {
                span: Span::from(0..11),
                elems: vec![
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
                ],
                puncts: vec![
                    OpAdd::from_span(3..4),
                    OpAdd::from_span(6..7),
                    OpAdd::from_span(10..11),
                ]
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
        let mut parser = Parser::new("(a)+(+(c)")?;
        //---------------------------------^ expecting Ident
        let result = parser.parse::<Punct2>()?;
        assert_eq!(
            result,
            Some(Punct2(tp::Punct {
                span: Span::from(0..9),
                elems: vec![
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
                ],
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

    #[test]
    fn parse_partial_item_recover_trailing() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(a)+(+")?;
        //---------------------------------^ expecting Ident
        let result = parser.parse::<Punct2>()?;
        assert_eq!(
            result,
            Some(Punct2(tp::Punct {
                span: Span::from(0..6),
                elems: vec![(
                    ParenOpen::from_span(0..1),
                    Ident::from_span(1..2),
                    ParenClose::from_span(2..3)
                ),],
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

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct PunctSame(tp::Punct<Ident, Ident>);

    #[test]
    fn parse_punct_same() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a b c d e")?;
        let result = parser.parse::<PunctSame>()?;
        assert_eq!(
            result,
            Some(PunctSame(tp::Punct {
                span: Span::from(0..9),
                elems: vec![
                    Ident::from_span(0..1),
                    Ident::from_span(4..5),
                    Ident::from_span(8..9),
                ],
                puncts: vec![Ident::from_span(2..3), Ident::from_span(6..7),]
            }))
        );
        assert_eq!(parser.remaining(), "");

        Ok(())
    }

    #[test]
    fn parse_punct_same_trailing() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a b c d e f")?;
        let result = parser.parse::<PunctSame>()?;
        assert_eq!(
            result,
            Some(PunctSame(tp::Punct {
                span: Span::from(0..11),
                elems: vec![
                    Ident::from_span(0..1),
                    Ident::from_span(4..5),
                    Ident::from_span(8..9),
                ],
                puncts: vec![
                    Ident::from_span(2..3),
                    Ident::from_span(6..7),
                    Ident::from_span(10..11),
                ]
            }))
        );
        assert_eq!(parser.remaining(), "");

        Ok(())
    }

    // note this is allowed for Split, but not for Punct
    // This is because in Split, after P, we know we need to parse another T
    // In Punct however, the P could be the trailing separator,
    // if T can be epsilon, we don't know if a None should be added.
    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq)]
    struct PunctOfOption(tp::Punct<tp::Option<Ident>, OpAdd>);

    #[test]
    fn punct_option_not_ll1() {
        assert_not_ll1!(
            PunctOfOption,
            GrammarError::FirstFirstConflict(
                "( (Ident)? )[+]*".to_string(),
                "( (Ident)? )[+]+".to_string(),
                "()".to_string(),
                "<empty>".to_string(),
            )
        );
    }

    // for similar reason, the separator cannot be optional
    // because we don't know if a trailing None should be added
    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq)]
    struct PunctOfOptionSep(tp::Punct<Ident, tp::Option<OpAdd>>);

    #[test]
    fn punct_option_sep_not_ll1() {
        assert_not_ll1!(
            PunctOfOptionSep,
            GrammarError::FirstFirstConflict(
                "((+)? ( Ident )[(+)?]*)?".to_string(),
                "(+)? ( Ident )[(+)?]*".to_string(),
                "()".to_string(),
                "<empty>".to_string(),
            )
        );
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    #[derive(Debug, PartialEq)]
    struct PunctBothOption(tp::Punct<tp::Option<Ident>, tp::Option<OpAdd>>);

    #[test]
    fn punct_both_option_conflict() {
        assert_not_ll1!(
            PunctBothOption,
            GrammarError::FirstFollowSeqConflict(
                "( (Ident)? )[(+)?]+".to_string(),
                "(Ident)?".to_string(),
                "((+)? ( (Ident)? )[(+)?]*)?".to_string(),
                "Ident".to_string()
            )
        );
    }
}
