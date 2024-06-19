use crate::syntax::{Metadata, Result as SynResult};
use crate::{Parser, Pos, Produce, Production, ToSpan};

use super::Node;

// When parser panics, skip tokens until an R can be parsed
#[derive(Debug, Clone, PartialEq)]
pub struct Recover<T: Produce, R: Produce>
    where R::Prod: Production<L = <T::Prod as Production>::L> 
    {
    pub head: Node<Option<T>>, 
    pub tail: R
    }

impl<T: Produce, R: Produce> ToSpan for Recover<T, R> 
    where R::Prod: Production<L = <T::Prod as Production>::L> 
{
    fn lo(&self) -> Pos {
        self.head.lo()
    }

    fn hi(&self) -> Pos {
        self.tail.hi()
    }
}

impl<T: Produce, R: Produce> Produce for Recover<T, R> 
    where R::Prod: Production<L = <T::Prod as Production>::L> 
{
    type Prod = (T::Prod, R::Prod);
    fn produce<'s>(
        parser: &mut Parser<'s, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        let mut errors = Vec::new();
        let head_first = meta.first.get(&T::prod_id());
        let token = parser.peek_token_src();
        let (mut panic, head_span, head) = if head_first.contains(token) || head_first.contains_epsilon() {
            match T::produce(parser, meta) {
                SynResult::Success(x) => (None, x.span(), Some(x)),
                SynResult::Recovered(x, e) => {
                    errors.extend(e);
                    (None, x.span(), Some(x))
                },
                SynResult::Panic(e) => {
                    (e.into_iter().next_back(), parser.current_span_empty(), None)
                }
            }
        } else {
            let e = parser.expecting(head_first.clone());
            (Some(e), parser.current_span_empty(), None)
        };
        let tail_first = meta.first.get(&R::prod_id());
        let tail = loop {
            let token = parser.peek_token_src();
            if token.is_none() {
                if tail_first.contains_epsilon() {
                    break match R::produce(parser, meta)  {
                        SynResult::Success(x) => Some(x),
                        SynResult::Recovered(x, e) => {
                            errors.extend(e);
                            Some(x)
                        },
                        SynResult::Panic(e) => {
                            if let Some(e) = e.into_iter().next_back() {
                                if let Some(p) = &mut panic {
                                    p.span.hi = e.span.hi;
                                } else {
                                    panic = Some(e);
                                }
                            }
                            None
                        }
                    
                    }
                }
                break None;
            }
            if tail_first.contains(token) || tail_first.contains_epsilon() {
                match R::produce(parser, meta)  {
                    SynResult::Success(x) => break Some(x),
                    SynResult::Recovered(x, e) => {
                        errors.extend(e);
                        break Some(x)
                    },
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
            }
            if let Some(p) = &mut panic {
                p.span.hi = parser.current_span().hi;
            } else {
                panic = Some(parser.expecting(tail_first.clone()));
            }
            parser.consume_token();
            
        };

        if let Some(panic) = panic {
            errors.push(panic);
        }

        match tail {
            Some(tail) => {
                let result = Self {
                    head: Node::new(head_span, head),
                    tail,
                };
                (result, errors).into()
            },
            None => {
                SynResult::Panic(errors)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax::{self, ErrorKind};
    use crate::prelude::*;

    use crate::test::{OpAdd, Ident, ParenOpen, ParenClose};
    use crate::test::MathTokenType as T;
    use crate::{Parser, GrammarError};

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct RecoverTest(tp::Recover<(ParenOpen, Ident, ParenClose), OpAdd>);

    #[test]
    fn parse_ok() -> Result<(), GrammarError> {
        let t = RecoverTest::parse("(a)+")?;
        assert_eq!(t, Some(RecoverTest(
            tp::Recover {
                head: Node::new(0..3, Some((
                    ParenOpen::from_span(0..1),
                    Ident::from_span(1..2),
                    ParenClose::from_span(2..3),
                ))),
                tail: OpAdd::from_span(3..4),
            }
        )));

        Ok(())
    }

    #[test]
    fn parse_recover_head_no_first_no_skip() -> Result<(), GrammarError> {
        let mut parser = Parser::new("+")?;
        //----------------------------^ expecting (
        let t = parser.parse::<RecoverTest>()?;
        assert_eq!(t, Some(RecoverTest(
            tp::Recover {
                head: Node::new(0..0, None),
                tail: OpAdd::from_span(0..1),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(0..1, ErrorKind::Expecting(first_set!(T{Paren:"("}))),
        ]);

        Ok(())
    }

    #[test]
    fn parse_recover_head_no_first_skip() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a)+")?;
        //----------------------------^^ expecting (
        let t = parser.parse::<RecoverTest>()?;
        assert_eq!(t, Some(RecoverTest(
            tp::Recover {
                head: Node::new(0..0, None),
                tail: OpAdd::from_span(2..3),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(0..2, ErrorKind::Expecting(first_set!(T{Paren:"("}))),
        ]);

        Ok(())
    }

    #[test]
    fn parse_recover_head_first_no_skip() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(+")?;
        //-----------------------------^ expecting Ident
        let t = parser.parse::<RecoverTest>()?;
        assert_eq!(t, Some(RecoverTest(
            tp::Recover {
                head: Node::new(1..1, None),
                tail: OpAdd::from_span(1..2),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(1..2, ErrorKind::Expecting(first_set!(T{Ident:*}))),
        ]);

        Ok(())
    }

    #[test]
    fn parse_recover_head_first_skip() -> Result<(), GrammarError> {
        let mut parser = Parser::new("()a+")?;
        //-----------------------------^^ expecting Ident
        let t = parser.parse::<RecoverTest>()?;
        assert_eq!(t, Some(RecoverTest(
            tp::Recover {
                head: Node::new(1..1, None),
                tail: OpAdd::from_span(3..4),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(1..3, ErrorKind::Expecting(first_set!(T{Ident:*}))),
        ]);

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    #[allow(clippy::type_complexity)]
    struct Recover2(tp::Recover<(ParenOpen, Ident, ParenClose), (ParenOpen, OpAdd, Ident, ParenClose)>);

    #[test]
    fn parse_panic_tail_no_first() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(a)+")?;
        //-------------------------------^ expecting (
        let t = parser.parse::<Recover2>()?;
        assert_eq!(t, None);
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(3..4, ErrorKind::Expecting(first_set!(T{Paren:"("}))),
        ]);
        assert_eq!(parser.remaining(), ""); // + is skipped attempting recovery

        Ok(())
    }

    #[test]
    fn parse_recover_tail_panic_no_skip() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(a)(+)(+b)")?;
        //---------------------------------^ expecting Ident
        let t = parser.parse::<Recover2>()?;
        assert_eq!(t, Some(Recover2(
            tp::Recover {
                head: Node::new(0..3, Some((
                    ParenOpen::from_span(0..1),
                    Ident::from_span(1..2),
                    ParenClose::from_span(2..3),
                ))),
                tail: (
                    ParenOpen::from_span(6..7),
                    OpAdd::from_span(7..8),
                    Ident::from_span(8..9),
                    ParenClose::from_span(9..10)),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(5..6, ErrorKind::Expecting(first_set!(T{Ident:*}))),
        ]);

        Ok(())
    }

    #[test]
    fn parse_recover_tail_panic_skip() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(a)(+)ccc(+b)")?;
        //---------------------------------^^^^ expecting Ident
        let t = parser.parse::<Recover2>()?;
        assert_eq!(t, Some(Recover2(
            tp::Recover {
                head: Node::new(0..3, Some((
                    ParenOpen::from_span(0..1),
                    Ident::from_span(1..2),
                    ParenClose::from_span(2..3),
                ))),
                tail: (
                    ParenOpen::from_span(9..10),
                    OpAdd::from_span(10..11),
                    Ident::from_span(11..12),
                    ParenClose::from_span(12..13)),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(5..9, ErrorKind::Expecting(first_set!(T{Ident:*}))),
        ]);

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct RecoverOption(tp::Recover<tp::Option<Ident>, OpAdd>);

    #[test]
    fn parse_option_none_ok() -> Result<(), GrammarError> {
        let mut parser = Parser::new("+")?;
        let t = parser.parse::<RecoverOption>()?;
        assert_eq!(t, Some(RecoverOption(
            tp::Recover {
                head: Node::new(0..0, Some(Node::new(0..0, None).into())),
                tail: OpAdd::from_span(0..1),
            }
        )));
        assert!(parser.info().errors.is_empty());
    
        Ok(())
    }
    
    #[test]
    fn parse_option_some_ok() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a+")?;
        let t = parser.parse::<RecoverOption>()?;
        assert_eq!(t, Some(RecoverOption(
            tp::Recover {
                head: Node::new(0..1, Some(Node::new(0..1, Some(Ident::from_span(0..1))).into())),
                tail: OpAdd::from_span(1..2),
            }
        )));
        assert!(parser.info().errors.is_empty());
    
        Ok(())
    }

    #[test]
    fn parse_option_some_skip_ok() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a((+")?;
        //-----------------------------^^ expecting +
        let t = parser.parse::<RecoverOption>()?;
        assert_eq!(t, Some(RecoverOption(
            tp::Recover {
                head: Node::new(0..1, Some(Node::new(0..1, Some(Ident::from_span(0..1))).into())),
                tail: OpAdd::from_span(3..4),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(1..3, ErrorKind::Expecting(first_set!(T{Op:"+"}))),
        ]);
    
        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct RecoverOption2(tp::Recover<Ident, tp::Option<OpAdd>>);

    #[test]
    fn parse_option2_none_ok() -> Result<(), GrammarError> {
        let mut parser = Parser::new("a")?;
        let t = parser.parse::<RecoverOption2>()?;
        assert_eq!(t, Some(RecoverOption2(
            tp::Recover {
                head: Node::new(0..1, Some(Ident::from_span(0..1))),
                tail: Node::new(1..1, None).into(),
            }
        )));
        assert!(parser.info().errors.is_empty());
    
        Ok(())
    }

    #[test]
    fn parse_option2_none_recover() -> Result<(), GrammarError> {
        let mut parser = Parser::new("(")?;
        let t = parser.parse::<RecoverOption2>()?;
        assert_eq!(t, Some(RecoverOption2(
            tp::Recover {
                head: Node::new(0..0, None),
                tail: Node::new(0..0, None).into(),
            }
        )));
        assert_eq!(parser.info().errors, vec![
            syntax::Error::new(0..1, ErrorKind::Expecting(first_set!(T{Ident:*}))),
        ]);
        assert_eq!(parser.remaining(), "(");
    
        Ok(())
    }
    
}
