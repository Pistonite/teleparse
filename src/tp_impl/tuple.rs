

// #[doc(hidden)]
// macro_rules! derive_tuple_pt {
//     (($($e:tt),*)) => {
// const _: () = {
//     #[automatically_derived]
//     impl<L: $crate::lex::Lexicon, 
//         $($e: $crate::parser::ParseTree,)*
//     > $crate::parser::ParseTree for ($($e,)*)
//     where
//         $(<$e as $crate::parser::ParseTree>::AST: $crate::syntax::AbstractSyntaxTree<L=L>,)*
//     {
//         type AST = ($(<$e>::AST,)*);
//
//         fn from_ast<'s>(ast: Self::AST, parser: &mut $crate::parser::Parser<'s, L>) -> Self {
//             #[allow(non_snake_case)]
//             let ($($e,)*) = ast;
//             ($(<$e>::from_ast($e, parser),)*)
//         }
//     }
// };
//
//     }
// }

use teleparse_macros::derive_tuple_production;

derive_tuple_production!{(A, B)}
derive_tuple_production!{(A, B, C)}
derive_tuple_production!{(A, B, C, D)}
derive_tuple_production!{(A, B, C, D, E)}

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
