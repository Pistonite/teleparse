//! string-based syntax tree nodes ([`Quote`], [`Parse`]
use std::marker::PhantomData;
use std::str::FromStr;

use crate::syntax::{Metadata, Result as SynResult};
use crate::{Parser, Produce, Production, ToSpan};

use super::Node;

/// Node that stores stringified source code
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Quote<S, T: Produce>(Node<S>, PhantomData<T>);

impl<S: std::fmt::Debug, T: Produce> std::fmt::Debug for Quote<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<S, T: Produce> Produce for Quote<S, T> 
    where S: for <'a> From<&'a str>
{
    type Prod = T::Prod;

    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        T::produce(parser, meta).map(|x| {
            let span = x.span();
            let src = parser.info().get_src(span);
            Node::new(span, S::from(src)).into()
        })
    }
}

/// Node that stores a parsed value from a string or the error if parse failed
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Parse<S: FromStr, T: Produce>(Node<Result<S, S::Err>>, PhantomData<T>);
impl<S: FromStr + std::fmt::Debug, T: Produce> std::fmt::Debug for Parse<S, T> 
    where S::Err: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<S: FromStr, T: Produce> Produce for Parse<S, T> 
{
    type Prod = T::Prod;

    fn produce(
        parser: &mut Parser<'_, <Self::Prod as Production>::L>,
        meta: &Metadata<<Self::Prod as Production>::L>,
    ) -> SynResult<Self, <Self::Prod as Production>::L> {
        T::produce(parser, meta).map(|x| {
            let span = x.span();
            let src = parser.info().get_src(span);
            Node::new(span, S::from_str(src)).into()
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::test::{Ident, Integer};
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct Stringified(tp::String<Ident>);
    
    #[test]
    fn test_stringify() {
        let t = Stringified::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..1 => \"a\"");
        assert_eq!(t, Stringified(Node::new(0..1, "a".to_string()).into()));
    }
    
    #[test]
    fn test_deref_string() {
        let t = Stringified::parse("a").unwrap().unwrap();
        let x: &String = &t.0;
        assert_eq!(x, "a");
        assert_eq!(&*t.0, "a");
    }
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq, Clone)]
    struct Parsed {
        ident: tp::Parse<u32, Ident>,
        num: tp::Parse<u32, Integer>,
        float: tp::Parse<f32, Integer>,
    }
    
    #[test]
    fn test_parse() {
        let t = Parsed::parse("abc 456 314").unwrap().unwrap();
        assert!(t.ident.is_err());
        assert_eq!(t.num, Node::new(4..7, Ok(456)).into());
        assert_eq!(t.float, Node::new(8..11, Ok(314.0)).into());
    
        assert_eq!(*t.num, Ok(456));
        assert_eq!(*t.float, Ok(314.0));
    }
}
