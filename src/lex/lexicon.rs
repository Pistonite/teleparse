use std::borrow::{Borrow, BorrowMut};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, Not};

use num::{Integer, Unsigned};

use crate::GrammarError;

use super::Lexer;

/// Trait for defining the token types of a grammar
///
/// See [module-level documentation](super) for more information
#[doc(alias = "TokenType")]
pub trait Lexicon: Debug + Clone + Copy + PartialEq + Eq + Hash + 'static {
    /// Bit flag representation of the token types
    ///
    /// When derived, One of u8, u16, u32, u64, or u128 is chosen based on the number of variants in the enum
    type Bit: Unsigned
        + Integer
        + BitAnd<Output = Self::Bit>
        + BitOr<Output = Self::Bit>
        + Not<Output = Self::Bit>
        + Copy;

    /// Lexer associated with this TokenType
    type Lexer<'s>: Lexer<'s, L = Self>;

    /// Map type used for storing token type mappings in tables for syntax analysis
    type Map<T: Default + Clone>: Default + Clone + Borrow<[T]> + BorrowMut<[T]>;

    /// Get the id of this token type (ordinal)
    fn id(&self) -> usize;
    fn from_id_unchecked(id: usize) -> Self;

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
    fn lexer(source: &str) -> Result<Self::Lexer<'_>, GrammarError>;
}
