

#[doc(hidden)]
macro_rules! derive_tuple_production {
    ($e1:tt, $($e:tt),*) => {
const _: () = {
    use $crate::syntax::Production;
    #[automatically_derived]
    impl<
        $e1: Production,
        $($e: Production<L=<$e1 as $crate::syntax::Production>::L>,)*
    > Production for ($e1, $($e),*)
    {
        type L = <$e1 as Production>::L;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            let mut s = ::std::string::String::new();//from("(");
            s.push_str(&<$e1>::debug()); 
        $( 
            s.push(' ');
            s.push_str(&<$e>::debug()); 
        )*
            // s.push(')');

            ::std::borrow::Cow::Owned(s)
        }
        fn register(meta: &mut $crate::syntax::MetadataBuilder<Self::L>) {
            $crate::register_sequence!(meta, $e1, $( $e ),*);
        }
    }
    impl<L: $crate::lex::Lexicon, 
        $e1: $crate::parser::Produce,
        $($e: $crate::parser::Produce,)*
    > $crate::parser::Produce for ($e1, $($e,)*)
    where
        <$e1 as $crate::parser::Produce>::Prod: Production<L=L>,
        $(<$e as $crate::parser::Produce>::Prod: Production<L=L>,)*
    {
        type Prod = (<$e1>::Prod, $(<$e>::Prod,)*);

        fn produce(
            parser: &mut $crate::parser::Parser<'_, <Self::Prod as Production>::L>, 
            meta: &$crate::syntax::Metadata<<Self::Prod as Production>::L>,
        ) -> $crate::syntax::Result<Self, <Self::Prod as Production>::L> {
            let mut errors = ::std::vec::Vec::new();
            let result = (
            $crate::handle_result!(errors, <$e1>::produce(parser, meta)),
        $(
            $crate::handle_result!(errors, <$e>::produce(parser, meta))
        ),*
            );
            (result, errors).into()
        }
    }
};

    }
}

derive_tuple_production!(A, B);
derive_tuple_production!(A, B, C);
derive_tuple_production!(A, B, C, D);
derive_tuple_production!(A, B, C, D, E);

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    
    use crate::lex::Token;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd, OpMul};
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct Tuple2(Ident, Ident);
    
    #[test]
    fn parse_tuple_2() {
        let t = Tuple2::parse("a b").unwrap().unwrap();
        assert_eq!(t, Tuple2(
            Ident(Token::new((0, 1), T::Ident)),
            Ident(Token::new((2, 3), T::Ident)),
        ));
    
        let t = Tuple2::parse("+").unwrap();
        assert!(t.is_none());
    
        let t = Tuple2::parse("a b c").unwrap().unwrap();
        assert_eq!(t, Tuple2(
            Ident(Token::new((0, 1), T::Ident)),
            Ident(Token::new((2, 3), T::Ident)),
        ));
    }
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct Tuple3(Ident, OpAdd, Ident);
    
    #[test]
    fn parse_tuple_3() {
        let t = Tuple3::parse("a+b").unwrap().unwrap();
        assert_eq!(t, Tuple3(
            Ident(Token::new((0, 1), T::Ident)),
            OpAdd(Token::new((1, 2), T::Op)),
            Ident(Token::new((2, 3), T::Ident)),
        ));
    }
    
    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct Nested {
        a: (Ident, OpMul, Ident),
        op: OpAdd,
        b: (Ident, OpMul, Ident),
    }
    
    #[test]
    fn parse_nested() {
        let t = Nested::parse("a*b + c*d").unwrap().unwrap();
        assert_eq!(t, Nested {
            a: (
                Ident(Token::new((0, 1), T::Ident)),
                OpMul(Token::new((1, 2), T::Op)),
                Ident(Token::new((2, 3), T::Ident)),
            ),
            op: OpAdd(Token::new((4, 5), T::Op)),
            b: (
                Ident(Token::new((6, 7), T::Ident)),
                OpMul(Token::new((7, 8), T::Op)),
                Ident(Token::new((8, 9), T::Ident)),
            ),
        });
    }
}
