use std::any::TypeId;

use crate::syntax::{self, Metadata, Production};
use crate::{GrammarError, ToSpan};

use super::Parser;

pub trait Produce: Sized + ToSpan {
    type Prod: Production;

    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>, 
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> syntax::Result<Self, <Self::Prod as Production>::L>;

    #[inline]
    fn prod_id() -> TypeId {
        <Self::Prod as Production>::id()
    }

}

pub trait Root: Produce
{
    fn parse(source: &str) -> Result<Option<Self>, GrammarError> {
        super::Parser::new(source)?.parse()
    }
    fn metadata() -> &'static Result<Metadata<<Self::Prod as Production>::L>, GrammarError>;

    fn assert_ll1() {
        if let Err(e) = Self::metadata() {
            panic!("{} is not LL(1): {}", Self::Prod::debug(), e);
        }
    }
}
