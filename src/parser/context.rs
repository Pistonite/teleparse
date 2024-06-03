

use std::ops::{Deref, DerefMut};

use crate::lex::{Token, TokenSrc, Lexer};
use crate::syntax::{Error, ErrorKind, FirstSet, FollowSet, Result as SynResult};
use crate::{AbstractSyntaxRoot, AbstractSyntaxTree, Lexicon, Span};

use super::{Info, ParseRoot, ParseTree, Parser};

pub struct Context<'p, 's, L: Lexicon> (&'p mut Parser<'s, L>);
impl <'p, 's, L: Lexicon> From<&'p mut Parser<'s, L>> for Context<'p, 's, L> {
    #[inline]
    fn from(parser: &'p mut Parser<'s, L>) -> Self {
        Self(parser)
    }
}

impl<'p, 's, L: Lexicon> Clone for Context<'p, 's, L> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<'p, 's, L: Lexicon> Deref for Context<'p, 's, L> {
    type Target = L::Ctx;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.context
    }
}

impl<'p, 's, L: Lexicon> DerefMut for Context<'p, 's, L> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.context
    }
}

impl<'p, 's, L: Lexicon> Context<'p, 's, L> {
    #[inline]
    pub fn info(&self) -> &Info<'s, L> {
        &self.0.info
    }
    #[inline]
    pub fn info_mut(&mut self) -> &mut Info<'s, L> {
        &mut self.0.info
    }
}
