#![doc = include_str!("../README.md")]

// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as teleparse;

// re-export user-facing proc-macros
pub use teleparse_macros::{derive_lexicon, derive_syntax};

/// prelude for all traits and common traits when working with this library
pub mod prelude {
    pub use crate::{derive_lexicon, derive_syntax};

    // traits
    pub use crate::ToSpan as _;
    pub use crate::Lexicon as _;
    pub use crate::Lexer as _;
    pub use crate::AbstractSyntaxTree as _;
    pub use crate::AbstractSyntaxRoot as _;
    pub use crate::ParseTree as _;
    pub use crate::ParseRoot as _;

    pub use crate::tp;
    // util
    pub use crate::{
        Pos, Span, Token, tp::Node,
    };

    // pub use crate::{derive_root, assert_ll1};

}

pub mod lex;
#[doc(inline)]
pub use lex::{ToSpan, Lexer, Lexicon, Pos, Span, Token};

pub mod syntax;
#[doc(inline)]
pub use syntax::{AbstractSyntaxTree, AbstractSyntaxRoot};

pub mod parser;
#[doc(inline)]
pub use parser::{Parser, ParseTree, ParseRoot};

pub mod tp;



#[cfg(test)]
pub(crate) mod test;

/// Error when constructing the grammar (i.e. not actually parsing yet).
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum GrammarError {
    #[error("Left recursion detected in the grammar! Stack: {0}")]
    LeftRecursion(String),
    #[error("The non-terminal `{0}` has a FIRST/FIRST conflict producing `{1}`. The conflicting terminals are: {2}")]
    FirstFirstConflict(String, String, String),
    #[error("The non-terminal `{0}` has a FIRST/FOLLOW conflict producing `{1}` followed by `{2}`. The conflicting terminals are: {3}")]
    FirstFollowSeqConflict(String, String, String, String),
    #[error("The non-terminal `{0}` has conflict in its FIRST and FOLLOW sets. The conflicting terminals are: {1}")]
    FirstFollowConflict(String, String),
}

impl GrammarError {
    pub fn left_recursion(stack: &[String], current: &str) -> Self {
        let message = format!("{} -> {}", stack.join(" -> "), current);
        Self::LeftRecursion(message)
    }
}

// private module for macros
#[doc(hidden)]
pub mod __priv {
    pub use teleparse_macros::*;
    pub use logos;
}
