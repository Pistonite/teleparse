//! Token related types and utils
//!
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Range;

use deref_derive::Deref;

mod token_set;
pub use token_set::*;
mod token_storage;
pub use token_storage::*;
mod token_type;
pub use token_type::*;

use crate::{SyntaxError, SyntaxErrorKind};

/// Position in the source code
pub type Pos = usize;

/// A arbitrary span in the source
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Start of the span, inclusive
    pub lo: Pos,
    /// End of the span, exclusive
    pub hi: Pos,
}

impl From<Range<Pos>> for Span {
    #[inline]
    fn from(range: Range<Pos>) -> Self {
        Self {
            lo: range.start,
            hi: range.end,
        }
    }
}

impl From<(Pos, Pos)> for Span {
    #[inline]
    fn from((lo, hi): (Pos, Pos)) -> Self {
        Self { lo, hi }
    }
}

impl Span {
    #[inline]
    pub fn new(lo: Pos, hi: Pos) -> Self {
        Self { lo, hi }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hi <= self.lo {
            write!(f, "{}", self.lo)
        } else {
            write!(f, "{}..{}", self.lo, self.hi)
        }
    }
}

/// One token in the lexer output
///
/// Each token stores the type and position of it in the source, but
/// not the actual source itself
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token<T: TokenType> {
    /// Position of this token in the source
    pub span: Span,
    /// Type of this token
    pub ty: T,
}

impl<T: TokenType> Debug for Token<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "token {:?}({:?})", self.ty, self.span)
    }
}

impl<T: TokenType> Token<T> {
    /// Create a new Token
    pub fn new<TSpan: Into<Span>>(span: TSpan, ty: T) -> Self {
        Self { span: span.into(), ty }
    }

    /// Get the source of this token
    pub fn get_src<'s>(&self, src: &'s str) -> &'s str {
        let Span { lo, hi } = self.span;
        if hi <= lo {
            return "";
        }
        let hi = hi.min(src.len());
        &src[lo..hi]
    }

    pub fn unexpected(self) -> SyntaxError {
        SyntaxError::new(self.span, SyntaxErrorKind::UnexpectedToken)
    }

    /// Associate source code with this token to make a [`SrcToken`]
    pub fn with_src<'t, 's, S: AsRef<str> + ?Sized>(&'t self, src: &'s S) -> SrcToken<'t, 's, T> {
        SrcToken::new(src.as_ref(), self)
    }
}

/// Associate a token with its source code
#[derive(Deref)]
pub struct SrcToken<'t, 's, T: TokenType> {
    pub src: &'s str,
    #[deref]
    pub token: &'t Token<T>,
}


impl<T: TokenType> Debug for SrcToken<'_, '_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "token {:?}\"{}\"({:?})", self.ty, self.as_str(), self.token.span)
    }
}

impl<'t, 's, T: TokenType> SrcToken<'t, 's, T> {
    #[inline]
    pub fn new(src: &'s str, token: &'t Token<T>) -> Self {
        Self { src: src.as_ref(), token }
    }

    /// Get the source str of this token
    pub fn as_str(&self) -> &str {
        self.token.get_src(self.src.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[crate::llnparse_derive(TokenType)]
    // pub enum TT {
    //     Comment,
    //     Keyword,
    // }
    //
    // #[test]
    // fn test_token() {
    //     let token = Token::new((0, 1), TT::Comment);
    //     assert_eq!(token.get_src("abc"), "a");
    // }
    //
    // #[test]
    // fn test_src_token() {
    //     let token = Token::new((1, 3), TT::Keyword);
    //     assert_eq!(token.with_src("abc").as_str(), "bc");
    // }
    //
    // #[test]
    // fn test_overflows() {
    //     let token = Token::new(0..100, TT::Comment);
    //     assert_eq!(token.get_src("abc"), "abc");
    // }
}
