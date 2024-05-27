//
//
// use crate::{Lexer, Parser, ParserState, SyntaxResult, SyntaxResultExt, SyntaxTreeParser, Token, TokenType};
// pub struct StringParser<'s, T: TokenType, In, Out> where
//     In: SyntaxTreeParser<'s, T=T,Target=Token<T>>,
//     Out: From<&'s str>,
//     {
//     inner: In,
//     _phantom: std::marker::PhantomData<&'s Out>,
// }
//
// impl<'s, T:TokenType, In: SyntaxTreeParser<'s, T=T,Target=Token<T>>, Out: From<&'s str>> 
//     StringParser<'s,T, In, Out> {
//     pub fn new(inner: In) -> Self {
//         Self {
//             inner,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }
//
// impl<'s, T:TokenType, In: SyntaxTreeParser<'s, T=T,Target=Token<T>>,
//     Out: From<&'s str>>
//     SyntaxTreeParser<'s> for StringParser<'s,T,In, Out> {
//     type T = T;
//     type Ctx = In::Ctx;
//     type Target = Out;
//
//     fn try_parse<L: Lexer<'s, T=Self::T>>(
//         &self, parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         self.inner.try_parse(parser).map_ext(|t| Out::from(parser.get_src(&t)))
//     }
// }
