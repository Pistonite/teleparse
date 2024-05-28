// use crate::{Lexer, Parser, ParserState, SyntaxErrors, SyntaxResult, SyntaxResultExt, SyntaxTreeParser};
//
// // use super::option::OptionParser;
// pub struct IterParser<'s, In, Out>
// where
//         In: SyntaxTreeParser<'s>,
//     {
//         allow_empty: bool,
//         inner: OptionParser<'s, In>,
//         _phantom: std::marker::PhantomData<&'s Out>,
//     }
// impl<'s, In: SyntaxTreeParser<'s>, Out> IterParser<'s, In, Out>
// {
//     fn parse_internal<L: Lexer<'s, T=In::T>>(&self, is_first: bool, parser: &mut Parser<'s, In::T, L, In::Ctx>) -> SyntaxResult<Option<In::Target>> {
//         if self.allow_empty || !is_first {
//             // use optional parser
//             self.inner.try_parse(parser)
//         } else {
//             self.inner.inner.try_parse(parser) .map_ext(Some)
//         }
//     }
//
// }
//
// impl<'s, In: SyntaxTreeParser<'s>, Out: FromIterator<In::Target>> SyntaxTreeParser<'s>
//     for IterParser<'s, In, Out>
// {
//     type T = In::T;
//     type Ctx = In::Ctx;
//     type Target = Out;
//
//     fn try_parse<L: Lexer<'s, T = Self::T>>(
//         &self,
//         parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         let (mut out, mut errors) = if !self.allow_empty {
//             // parse the first with required parser
//             match self.inner.inner.try_parse(parser) {
//                 Ok(first) => (vec![first], vec![]),
//                 Err(err) => {
//                     // partial success
//                     let (value, errors) = err.unwrap_or_into()?;
//                     (vec![value], errors)
//                 }
//             }
//         } else {
//             (vec![], vec![])
//         };
//         loop {
//             match self.inner.try_parse(parser) {
//                 Ok(None) => break,
//                 Ok(Some(value)) => out.push(value),
//                 Err(err) => {
//                     match err.obj.flatten() {
//                         Some(value) => {
//                             out.push(value);
//                             errors.extend(err.errors);
//                         }
//                         None => break,
//                     }
//                 }
//             }
//         }
//
//         let out = Out::from_iter(out);
//         if errors.is_empty() {
//             Ok(out)
//         } else {
//             Err(SyntaxErrors::new(Some(out), errors))
//         }
//     }
// }
//
