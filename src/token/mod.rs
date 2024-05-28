//! Token related types and utils
//!
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Range, BitAnd, BitOr, Not};

use num::{Integer, Unsigned};

mod token_set;
pub use token_set::*;
mod token_storage;
pub use token_storage::*;

use crate::{Lexer, SyntaxError, SyntaxErrorKind};

/// Trait for token types
///
/// ## Note
/// This is normally derived with [`#[teleparse_derive(TokenType)]`](crate::teleparse_derive) on an enum
/// instead of manually implementing it. The macro will derive the token type and lexer according
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
/// ## Enum Attributes
/// These attributes apply to the enum
/// #### `ignore`
/// This is used to define patterns to ignore when lexing. A common example is whitespaces.
/// The pattern is a regular expression that must start with "^" and does not match the empty
/// string.
/// ```no_compile
/// #[teleparse_derive(TokenType)]
/// #[teleparse(ignore(r#"^\s+"#))]
/// pub enum MyToken {
///     ...
/// }
/// ```
/// If the pattern is complicated, you can use multiple regular expressions are well. Use `,` to separate them (i.e. `ignore(r#"^\s+"#, r#"^\d+"#)`)
///
/// ## Variant Attributes
/// These attributes apply to the variants
/// #### `terminal`
/// Used to generate terminal [`SyntaxTree`](crate::SyntaxTree) implementation for tokens. There
/// are 2 forms:
/// - `terminal(XXX)` for generating a `XXX` SyntaxTree node that will match the variant token type
/// - `terminal(XXX = "literal")` for generating a `XXX` SyntaxTree node that will match the variant token type only if the token content is exactly `"literal"`
///
/// If any terminal does not have a literal value to match, you also need a `regex` attribute to
/// define the regex to match the token
///
/// #### `regex`
/// Used to define the 
pub trait TokenType: Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// Bitflag representation of the token type. This could be u8, u16, u32, u64, or u128
    type Repr: Unsigned + Integer + BitAnd<Output = Self::Repr> + BitOr<Output = Self::Repr> + Not<Output = Self::Repr> + Copy;

    /// Lexer associated with this TokenType
    type Lexer<'s>: Lexer<'s, T = Self>;

    /// Whether this token should be excluded from AST, but still has value.
    ///
    /// One example is comments
    fn should_extract(&self) -> bool;

    /// Convert to numeric representation for use in bit set
    fn to_repr(&self) -> Self::Repr;

    /// Get the first type. Used to iterate over all types
    fn first() -> Self;

    /// Get the next type. Used to iterate over all types
    fn next(&self) -> Option<Self>;

    /// Create a lexer for parsing this token type
    fn lexer<'s>(source: &'s str) -> Self::Lexer<'s>;
}

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

    /// Create an unexpected token error
    pub fn unexpected(self) -> SyntaxError {
        SyntaxError::new(self.span, SyntaxErrorKind::UnexpectedToken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[crate::teleparse_derive(TokenType)]
    pub enum TT {
        #[teleparse(regex(r"^//.*"))]
        Comment,
        #[teleparse(terminal(Keyword = r"fn"))]
        Keyword,
    }
    #[test]
    fn test_token() {
        let token = Token::new((0, 1), TT::Comment);
        assert_eq!(token.get_src("abc"), "a");
    }

    #[test]
    fn test_empty() {
        let token = Token::new((0, 0), TT::Comment);
        assert_eq!(token.get_src("abc"), "");
        let token = Token::new((1, 1), TT::Comment);
        assert_eq!(token.get_src("abc"), "");
        let token = Token::new((1, 0), TT::Comment);
        assert_eq!(token.get_src("abc"), "");
    }
    
    #[test]
    fn test_overflows() {
        let token = Token::new(0..100, TT::Comment);
        assert_eq!(token.get_src("abc"), "abc");
    }
}
