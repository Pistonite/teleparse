use crate::production_passthrough;
use crate::syntax::{Metadata, Result as SynResult};
use crate::{Parser, Pos, Produce, Production, Span, ToSpan};

impl<T: Production> Production for Box<T> {
    production_passthrough!(T);
}

impl<T: ToSpan> ToSpan for Box<T> {
    fn lo(&self) -> Pos {
        self.as_ref().lo()
    }
    fn hi(&self) -> Pos {
        self.as_ref().hi()
    }
    fn span(&self) -> Span {
        self.as_ref().span()
    }
}

impl<T: Produce> Produce for Box<T> {
    type Prod = Box<T::Prod>;

    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        T::produce(parser, meta).map(Box::new)
    }
}
