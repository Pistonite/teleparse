use std::io::ErrorKind;

use crate::syntax::{self, Epsilon, Metadata, MetadataBuilder, Result as SynResult};
use crate::{Parser, Pos, Produce, Production, ToSpan};

use super::option::OptionProd;
use super::Node;


// When parser panics, skip tokens until an R can be parsed
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
        let (mut panic, head_span, head) = match T::produce(parser, meta) {
            SynResult::Success(x) => (None, x.span(), Some(x)),
            SynResult::Recovered(x, e) => {
                errors.extend(e);
                (None, x.span(), Some(x))
            },
            SynResult::Panic(e) => {
                (e.into_iter().rev().next(), parser.current_span_empty(), None)
            }
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
                            if let Some(e) = e.into_iter().rev().next() {
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
            if tail_first.contains(token) {
                match R::produce(parser, meta)  {
                    SynResult::Success(x) => break Some(x),
                    SynResult::Recovered(x, e) => {
                        errors.extend(e);
                        break Some(x)
                    },
                    SynResult::Panic(e) => {
                        if let Some(e) = e.into_iter().rev().next() {
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
