use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, Not};

use num::{Integer, Unsigned};

use crate::Lexer;

/// Trait for token types
///
/// ## Note
/// This is normally derived with [`#[llnparse_derive(TokenType)]`](crate::llnparse_derive) on an enum
/// instead of manually implementing it. This is a shorthand
/// for deriving [`TokenType`], other required traits, and generating a bit set
/// implementation for use as semantics.
///
/// Appriopriate size would be chosen automatically for the underlying representation:
/// `u8`, `u16`, `u32`, `u64`, or `u128` depending on the number of variants. You can
/// have at most 128 token types (which should be plenty).
///
/// ## Example
/// ```rust
#[doc = include_str!("../../tests/expand/token_type_example.rs")]
/// ```
pub trait TokenType: Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// Bitflag representation of the token type. This could be u8, u16, u32, u64, or u128
    type Repr: Unsigned + Integer + BitAnd<Output = Self::Repr> + BitOr<Output = Self::Repr> + Not<Output = Self::Repr> + Copy;

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

