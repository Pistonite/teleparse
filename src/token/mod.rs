//! Token related types and utils
//!
use teleparse_macros::ToSpan;

use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, Not, Range};

use num::{Integer, Unsigned};

mod map;
pub use map::*;
mod token_set;
pub use token_set::*;
mod token_storage;
pub use token_storage::*;

use crate::{Lexer, Error, ErrorKind};

/// Trait for token types, derivable with [`#[teleparse_derive(TokenType)]`](crate::teleparse_derive)
///
/// ## Note
/// This is normally derived from an enum instead of manually implementing it. The macro will derive the token type and lexer according
/// to the attributes
///
/// Appriopriate size would be chosen automatically for the underlying representation:
/// `u8`, `u16`, `u32`, `u64`, or `u128` depending on the number of variants. You can
/// have at most 128 token types (which should be plenty).
///
/// ## Example
/// This is a full example. See below for more details
/// ```rust
#[doc = include_str!("../../tests/expand/token_type_example.rs")]
/// ```
///
#[doc = include_str!("./README.md")]
pub trait TokenType: Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// Bitflag representation of the token type. This could be u8, u16, u32, u64, or u128
    type Bit: Unsigned + Integer + BitAnd<Output = Self::Bit> + BitOr<Output = Self::Bit> + Not<Output = Self::Bit> + Copy;

    /// Lexer associated with this TokenType
    type Lexer<'s>: Lexer<'s, T = Self>;

    type Map<T: Default + Clone>: Default + Clone + Borrow<[T]> + BorrowMut<[T]>;

    /// Context type associated with parsing this TokenType and SyntaxTree
    type Ctx;

    /// Get the id of this token type (ordinal)
    fn id(&self) -> usize;
    fn from_id(id: usize) -> Self;

    /// Convert to numeric representation for use in bit set
    fn to_bit(&self) -> Self::Bit;

    /// Get the first type. Used to iterate over all types
    fn first() -> Self;

    /// Get the next type. Used to iterate over all types
    fn next(&self) -> Option<Self>;

    /// Whether this token should be excluded from AST, but still has value.
    ///
    /// One example is comments
    fn should_extract(&self) -> bool;

    /// Create a lexer for parsing this token type
    fn lexer<'s>(source: &'s str) -> Self::Lexer<'s>;

    // /// Create a parser for parsing syntax trees with this token type
    // fn parser_with_context<'s>(source: &'s str, context: Self::Ctx) -> Parser<'s, Self> {
    //     Parser::new_with_context(source, context)
    // }
}

/// Convienence trait for token types without context
///
/// This is automatically derived for any [`TokenType`] that has `()` as the context
pub trait TokenTypeNoCtx: TokenType<Ctx=()> {
    // /// Create a parser for parsing syntax trees with this token type
    // fn parser<'s>(source: &'s str) -> Parser<'s, Self> {
    //     Self::parser_with_context(source, ())
    // }
}

impl<T: TokenType<Ctx=()>> TokenTypeNoCtx for T {}

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
    pub fn get_src<'s>(&self, src: &'s str) -> &'s str {
        if self.hi <= self.lo {
            return "";
        }
        let hi = self.hi.min(src.len());
        &src[self.lo..hi]
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

/// Trait for types that can be converted to a [`Span`]
///
/// Tokens and derived SyntaxTree nodes all implement this trait
pub trait ToSpan {
    /// Get the span of this type
    fn span(&self) -> Span;
}

impl ToSpan for Span {
    #[inline]
    fn span(&self) -> Span {
        *self
    }
}

impl<A, B> ToSpan for (A, B)
where
    A: ToSpan,
    B: ToSpan,
{
    #[inline]
    fn span(&self) -> Span {
        (self.0.span().lo, self.1.span().hi).into()
    }
}

/// One token in the lexer output
///
/// Each token stores the type and position of it in the source, but
/// not the actual source itself
#[derive(Clone, Copy, PartialEq, Eq, Hash, ToSpan)]
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
    #[inline]
    pub fn get_src<'s>(&self, src: &'s str) -> &'s str {
        self.span.get_src(src)
    }

    /// Get the content of this token as a `TokenSrc`
    #[inline]
    pub fn to_src<'s>(&self, src: &'s str) -> TokenSrc<'s, T> {
        TokenSrc {
            ty: self.ty,
            src: self.get_src(src),
        }
    }

    // /// Create an unexpected token error
    // pub fn unexpected(self) -> SyntaxError<T> {
    //     SyntaxError::new(self.span, SyntaxErrorKind::UnexpectedToken)
    // }
}

/// Holder of a token type + the source code spanning that token
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenSrc<'s, T: TokenType> {
    pub ty: T,
    pub src: &'s str,
}

impl<'s, T: TokenType> TokenSrc<'s, T> {
    #[inline]
    pub fn new(ty: T, src: &'s str) -> Self {
        Self { ty, src }
    }
}

impl<'s, T: TokenType> From<(T, &'s str)> for TokenSrc<'s, T> {
    #[inline]
    fn from((ty, src): (T, &'s str)) -> Self {
        Self { ty, src }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[crate::teleparse_derive(TokenType)]
    // pub enum TT {
    //     #[teleparse(regex(r"^//.*"))]
    //     Comment,
    //     #[teleparse(terminal(Keyword = r"fn"))]
    //     Keyword,
    // }
    // #[test]
    // fn test_token() {
    //     let token = Token::new((0, 1), TT::Comment);
    //     assert_eq!(token.get_src("abc"), "a");
    // }
    //
    // #[test]
    // fn test_empty() {
    //     let token = Token::new((0, 0), TT::Comment);
    //     assert_eq!(token.get_src("abc"), "");
    //     let token = Token::new((1, 1), TT::Comment);
    //     assert_eq!(token.get_src("abc"), "");
    //     let token = Token::new((1, 0), TT::Comment);
    //     assert_eq!(token.get_src("abc"), "");
    // }
    //
    // #[test]
    // fn test_overflows() {
    //     let token = Token::new(0..100, TT::Comment);
    //     assert_eq!(token.get_src("abc"), "abc");
    // }
}
