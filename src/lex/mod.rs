//! # Lexical Analysis
//!
//! This module implements token-related utilities used
//! for [`#[derive_lexicon]`](crate::derive_lexicon).
//! You can read more in the [teleparse
//! book](https://teleparse.pistonite.org/lexical_analysis/derive_lexicon_g.html)
//!
//! ## Example
//! ```rust
#![doc = include_str!("../../tests/expand/lexicon_example.rs")]
//! ```

mod lexer;
pub use lexer::*;
mod lexicon;
pub use lexicon::*;
mod map;
pub use map::*;
mod set;
pub use set::*;
mod span;
pub use span::*;
mod token;
pub use token::*;
mod vec;
pub use vec::*;

