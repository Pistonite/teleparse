//! optional syntax tree nodes ([`Option`], [`Exists`])
use std::{borrow::Cow, marker::PhantomData};


use teleparse_macros::derive_production;

use crate::{syntax::{Epsilon, Metadata, MetadataBuilder, Result as SynResult}, Lexicon, Parser, Pos, Produce, Production, ToSpan};

use super::{Node};

// Option<T> => T | epsilon
#[doc(hidden)]
pub enum OptionProd<T: Production> {
    Some(T),
    None(Epsilon<T::L>),
}

impl<T: Production> Production for OptionProd<T> {
    type L = T::L;
    #[inline]
    fn debug() -> Cow<'static, str> {
        Cow::Owned(format!("Option<{}>", T::debug()))
    }

    fn register(meta: &mut MetadataBuilder<Self::L>) {
        let t = Self::id();
        if meta.visit(t, ||Self::debug().into_owned()) {
            meta.add_union(t, &[T::id(), Epsilon::<T::L>::id()]);
            T::register(meta);
            Epsilon::<T::L>::register(meta);
        }
    }
}

impl<T: Production> ToSpan for OptionProd<T> {
    fn lo(&self) -> Pos {
        match self {
            OptionProd::Some(t) => t.lo(),
            OptionProd::None(e) => e.lo(),
        }
    }
    fn hi(&self) -> Pos {
        match self {
            OptionProd::Some(t) => t.hi(),
            OptionProd::None(e) => e.hi(),
        }
    }
}

/// Node that stores an optional subtree `Option<T> => T | epsilon`
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Optional<T: Produce>(Node<Option<T>>);

impl<T: std::fmt::Debug + Produce> std::fmt::Debug for Optional<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0.value {
            Some(node) => {
                f.debug_tuple("Some").field(&node).finish()
            }
            None => {
                f.debug_tuple("None").field(&self.0.span).finish()
            }
        }
    }
}

// impl<T: Produce> From<OptionProd<T::Prod>> for Optional<T> {
//     fn from(prod: OptionProd<T::Prod>) -> Self {
//         match prod {
//             OptionProd::Some(t) => Node::new(t.span(), Some(t)).into(),
//             OptionProd::None(e) => Node::new(e.span(), None).into(),
//         }
//     }
// }

impl<T: Produce> Produce for Optional<T> {
    type Prod = OptionProd<T::Prod>;
    fn produce<'s>(
        parser: &mut Parser<'s, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        produce_option(parser,meta,|x|x).map(Self::from)
    }

}

/// Node that stores if an optional subtree is produced
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Exists<T: Produce>(Node<bool>, PhantomData<T>);

impl<T: Produce> std::fmt::Debug for Exists<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Produce> Produce for Exists<T> {
    type Prod = OptionProd<T::Prod>;
    fn produce<'s>(
        parser: &mut Parser<'s, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        produce_option(parser,meta,|x: Option<T>|x.is_some()).map(Self::from)
    }

}
    fn produce_option<'s, T , O, F: FnOnce(Option<T>) -> O, L: Lexicon>(
        parser: &mut Parser<'s, L>,
        meta: &Metadata<L>,
f: F
    ) -> SynResult<Node<O>, L>
where T: Produce + ToSpan,
T::Prod: Production<L = L>,
{
        let token = parser.peek_token_src();
        if token.is_none() {
            // produces epsilon
            return SynResult::Success(
                Node::new(parser.current_span_empty(), f(None))
            );
        }
        let first = meta.first.get(&T::prod_id());
        if !first.contains(token) {
            // produces epsilon
            return SynResult::Success(
                Node::new(parser.current_span_empty(), f(None))
            );
        }
        // if parse fails, delay to parent to panic
        match T::produce(parser, meta) {
            SynResult::Success(t) => {
                SynResult::Success(Node::new(t.span(), f(Some(t))))
            },
            SynResult::Recovered(t, error) =>{
                SynResult::Recovered(Node::new(t.span(), f(Some(t))), error)
            }
            SynResult::Panic(error) => {
                SynResult::Recovered(
                        Node::new(parser.current_span_empty(), f(None))
                        , error
                    )
            }
        }
    }

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::GrammarError;

    use crate::lex::Token;
    use crate::test::prelude::*;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd};

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct OptIdent(tp::Option<Ident>);

    #[test]
    fn test_none() -> Result<(), GrammarError> {
        let t = OptIdent::parse("+")?.unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "None(0)");
        assert_eq!(t, OptIdent(Node::new(0..0, None).into()));

        Ok(())
    }

    #[test]
    fn test_some() {
        let t = OptIdent::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "Some(token Ident(0..1))");
        assert_eq!(t, OptIdent(Node::new(0..1, Some(
            Ident(Token::new(0..1, T::Ident))
        )).into()));
    }

    #[test]
    fn test_use_as_option() -> Result<(), GrammarError> {
        let t = OptIdent::parse("a")?.unwrap();
        assert!(t.0.is_some());

        Ok(())
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    struct Seq(tp::Option<OpAdd>, OpAdd);

    #[test]
    fn test_seq_not_ll1() {
        assert_not_ll1!(Seq, GrammarError::FirstFollowSeqConflict(
            "Seq".to_string(),
            "Option<OpAdd>".to_string(),
            "OpAdd".to_string(),
            "\"+\"".to_string()
        ));
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    struct Nested(super::Optional<super::Optional<Ident>>);

    #[test]
    fn test_nested_not_ll1() {
        assert_not_ll1!(Nested, GrammarError::FirstFirstConflict(
            "Option<Option<Ident>>".to_string(),
            "epsilon".to_string(),
            "<empty>".to_string(),
        ));
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct ExistIdent(tp::Exists<Ident>);

    #[test]
    fn parse_exist() {
        let t = ExistIdent::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..1 => true");
        assert_eq!(t, ExistIdent(Node::new(0..1, true).into()));
        assert_eq!(*t.0, true);

        let t = ExistIdent::parse("+").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0 => false");
        assert_eq!(t, ExistIdent(Node::new(0..0, false).into()));
        assert_eq!(*t.0, false);
    }
}
