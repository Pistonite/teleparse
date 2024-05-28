use std::marker::PhantomData;

use deref_derive::{Deref, DerefMut};

use crate::{Lexer, Parser, ParserState, Span, SyntaxResult, SyntaxResultExt, SyntaxTree, SyntaxTreeParser};

use super::node::Node;

#[derive(Deref, DerefMut)]
pub struct Optional<T>(#[deref] Node<Option<T>>);

impl<ST: SyntaxTree> SyntaxTree for Optional<ST> {
    type T = ST::T;
    type Ctx = ST::Ctx;
    type AST = Result<ST::AST, Span>;

    #[inline]
    fn span_of(ast: &Self::AST) -> Span {
        match ast {
            Ok(ast) => ST::span_of(ast),
            Err(span) => *span,
        }
    }

    fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> SyntaxResult<Self::AST> {
        parser.push_state().map_ext(|_| {
            let result = ST::try_parse_ast(parser).into_value();
            match result {
                Some(ast) => {
                    parser.pop_state();
                    Ok(ast)
                }
                None => {
                    let e = Err(parser.current_span());
                    parser.restore_state();
                    parser.pop_state();
                    e
                }
            }
        })
    }

    #[inline]
    fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
        ast: Self::AST,
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> Self {
        match ast {
            Ok(ast) => Self(Node::new(ST::span_of(&ast), Some(ST::into_parse_tree(ast, parser)))),
            Err(span) => Self(Node::new(span, None)),
        }
    }
}


#[derive(Deref, DerefMut)]
pub struct Exists<T>(#[deref] Node<bool>, PhantomData<T>);
impl<T> Exists<T> {
    #[inline]
    pub fn new(span: Span, exists: bool) -> Self {
        Self(Node::new(span, exists), PhantomData)
    }
    #[inline]
    pub fn exists(&self) -> bool {
        self.0.value
    }
}
impl<ST: SyntaxTree> SyntaxTree for Exists<ST> {
    type T = ST::T;
    type Ctx = ST::Ctx;
    type AST = Result<ST::AST, Span>;

    #[inline]
    fn span_of(ast: &Self::AST) -> Span {
        <Optional<ST> as SyntaxTree>::span_of(ast)
    }

    #[inline]
    fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> SyntaxResult<Self::AST> {
        <Optional<ST> as SyntaxTree>::try_parse_ast(parser)
    }

    #[inline]
    fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
        ast: Self::AST,
        parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
    ) -> Self {
        match ast {
            Ok(ast) => {
                let span = ST::span_of(&ast);
                // create the parse tree but throw it away
                ST::into_parse_tree(ast, parser);
                Self::new(span, true)
            }
            Err(span) => Self::new(span, false),
        }
    }
}
//
// impl<T> Exists<T> {
//     #[inline]
//     pub fn as_option(&self) -> Option<&T> {
//         self.0.as_ref()
//     }
// }
//
// impl<T> Deref for Exists<T> {
//     type Target = bool;
//
//     fn deref(&self) -> &bool {
//         if self.0.is_some() {
//             &true
//         } else {
//             &false
//         }
//     }
// }


// /// Parser for `#[llnparse(optional)]`, or inferred from [`Option`]
// pub struct OptionParser<'s, In> where
//     In: SyntaxTreeParser<'s>,
//     {
//     pub inner: In,
//     _phantom: std::marker::PhantomData<&'s ()>,
// }
//
// impl<'s, In: SyntaxTreeParser<'s>> OptionParser<'s, In> {
//     pub fn new(inner: In) -> Self {
//         Self {
//             inner,
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }
//
// impl<'s, In: SyntaxTreeParser<'s>> SyntaxTreeParser<'s> for OptionParser<'s, In> {
//     type T = In::T;
//     type Ctx = In::Ctx;
//     type Target = Option<In::Target>;
//
//     fn try_parse<L: Lexer<'s, T=Self::T>>(
//         &self, parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         parser.push_state().map_ext(|_| {
//             let result = self.inner.try_parse(parser).into_value();
//             if result.is_none() {
//                 parser.restore_state();
//             }
//             parser.pop_state();
//             result
//         })
//     }
// }
//
// /// Parser for `#[llnparse(presence)]`, or inferred from `bool`
// pub struct PresenceParser<'s, In> where
//     In: SyntaxTreeParser<'s>,
//     {
//     inner: OptionParser<'s, In>,
//     _phantom: std::marker::PhantomData<&'s ()>,
// }
//
// impl<'s, In: SyntaxTreeParser<'s>> PresenceParser<'s, In> {
//     pub fn new(inner: In) -> Self {
//         Self {
//             inner: OptionParser::new(inner),
//             _phantom: std::marker::PhantomData,
//         }
//     }
// }
//
// impl<'s, In: SyntaxTreeParser<'s>> SyntaxTreeParser<'s> for PresenceParser<'s, In> {
//     type T = In::T;
//     type Ctx = In::Ctx;
//     type Target = bool;
//
//     fn try_parse<L: Lexer<'s, T=Self::T>>(
//         &self, parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
//     ) -> SyntaxResult<Self::Target> {
//         self.inner.try_parse(parser).map_ext(|x| x.is_some())
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_optional_as_option() {
        let o = Optional(Node::new(Span::new(0, 0), Some(42)));
        assert_eq!(o.as_ref().copied(), Some(42));
        let opt: &Option<i32> = &o;
        assert_eq!(opt.as_ref().copied(), Some(42));
    }
    #[test]
    fn use_exists() {
        let e = Exists::<String>::new(Span::new(0, 0), true);
        assert!(e.exists());
    }
}
