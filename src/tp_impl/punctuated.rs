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
        // let (first_elem, mut errors) = match T::produce(parser, meta) {
        //     SynResult::Success(t) => {
        //         (t, Vec::new())
        //     }
        //     SynResult::Recovered(t, e) => {
        //         (t, e)
        //     }
        //     SynResult::Panic(e) => {
        //         return SynResult::Panic(e);
        //     }
        // };
        // let mut hi = first_elem.hi();
        // let produce_iter = ProduceIter::new(
        //     parser, meta, 
        //     meta.first.get(&Self::prod_id()), &mut errors, &mut hi);
        // let lo = first_elem.lo();
        // let v: V = std::iter::once(first_elem)
        //     .chain(produce_iter)
        //     .collect();
        // (Node::new(lo..hi, v).into(), errors).into()
        todo!()
    }

}
