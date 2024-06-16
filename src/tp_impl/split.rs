use std::ops::{Deref, DerefMut};

use crate::{syntax::{Metadata, Result as SynResult}, Lexicon, Parser, Produce, Production, ToSpan};

use super::{iter::OneOrMore, option::OptionProd, Node};


// Split<T> => T (P T)*
#[doc(hidden)]
#[derive(ToSpan, Debug, Clone, PartialEq)]
pub struct Split<T: Produce, P: Produce> {
    pub elems: Node<Vec<T>>,
    pub puncts: Vec<P>,
}

impl<T: Produce, P: Produce> Deref for Split<T, P> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.elems
    }
}

impl<T: Produce, P: Produce> DerefMut for Split<T, P> 
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elems
    }
}

impl<L: Lexicon, T: Produce, P: Produce> Produce for Split<T, P> 
    where T::Prod: Production<L = L>,
          P::Prod: Production<L = L>,
{
    type Prod = (T::Prod, OptionProd<OneOrMore<(P::Prod, T::Prod)>>);
    fn produce<'s>(
        parser: &mut Parser<'s, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        let (lo, mut elems, mut errors) = match T::produce(parser, meta) {
            SynResult::Success(t) => {
                (t.lo(), vec![t], Vec::new())
            }
            SynResult::Recovered(t, e) => {
                (t.lo(), vec![t], e)
            }
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
            loop {
                let mut token = parser.peek_token_src();
                if elem_first.contains(token) {
                    match T::produce(parser, meta) {
                        SynResult::Success(t) => {
                            elems.push(t);
                            puncts.push(p);
                            continue 'outer;
                        },
                        SynResult::Recovered(t, e) => {
                            elems.push(t);
                            puncts.push(p);
                            errors.extend(e);
                            continue 'outer;
                        },
                        SynResult::Panic(e) => {
                            errors.extend(e);
                        }
                    }
                    // recovery
                    token = parser.peek_token_src();
                } else {
                    errors.push(
                        parser.expecting(elem_first.clone())
                    );
                }
                if punct_first.contains(token) {
                    continue 'outer;
                }
                if token.is_none() || follow.contains(token) {
                    break 'outer;
                }
                parser.consume_token();
            }
        }
        let hi = elems.last().map_or_else(|| lo, |t| t.hi());
        let s = Self { elems: Node::new(lo..hi, elems), puncts };
        (s, errors).into()
    }

}
