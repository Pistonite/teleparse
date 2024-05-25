use std::fmt::Debug;
use std::hash::Hash;

/// Trait for token types
///
/// ## Note
/// This is normally derived with [`#[llnparse_derive(TokenType)]`](llnparse_derive) on an enum
/// instead of manually implementing it. This is a shorthand
/// for deriving [`TokenType`] and other required traits.
///
/// ## Example
/// ```rust
#[doc = include_str!("../../tests/expand/token_type.rs")]
/// ```
pub trait TokenType: Debug + Clone + Copy + PartialEq + Eq + Hash {
    /// Whether this token should be excluded from AST, but still has value.
    ///
    /// One example is comments
    fn should_extract(&self) -> bool;
}
