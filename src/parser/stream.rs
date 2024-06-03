
use crate::lex::{Token, TokenSrc, Lexer};
use crate::syntax::{Error, ErrorKind, FirstSet, FollowSet, Result as SynResult};
use crate::{Lexicon, Span};

use super::Parser;

pub struct Stream<'p, 's, L: Lexicon>(&'p mut Parser<'s, L>);

impl <'p, 's, L: Lexicon> From<&'p mut Parser<'s, L>> for Stream<'p, 's, L> {
    #[inline]
    fn from(parser: &'p mut Parser<'s, L>) -> Self {
        Self(parser)
    }
}

impl<'p, 's, L: Lexicon> Stream<'p, 's, L> {


}
