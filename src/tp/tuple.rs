

#[doc(hidden)]
macro_rules! derive_tuple_pt {
    (($($e:tt),*)) => {
const _: () = {
    #[automatically_derived]
    impl<L: $crate::lex::Lexicon, 
        $($e: $crate::parser::ParseTree,)*
    > $crate::parser::ParseTree for ($($e,)*)
    where
        $(<$e as $crate::parser::ParseTree>::AST: $crate::syntax::AbstractSyntaxTree<L=L>,)*
    {
        type AST = ($(<$e>::AST,)*);

        fn from_ast<'s>(ast: Self::AST, parser: &mut $crate::parser::Parser<'s, L>) -> Self {
            #[allow(non_snake_case)]
            let ($($e,)*) = ast;
            ($(<$e>::from_ast($e, parser),)*)
        }
    }
};

    }
}

use teleparse_macros::derive_tuple_ast;

derive_tuple_ast!{(A, B)}
derive_tuple_ast!{(A, B, C)}
derive_tuple_ast!{(A, B, C, D)}
derive_tuple_ast!{(A, B, C, D, E)}
derive_tuple_pt!{(A, B)}
derive_tuple_pt!{(A, B, C)}
derive_tuple_pt!{(A, B, C, D)}
derive_tuple_pt!{(A, B, C, D, E)}
