#![doc = include_str!("../README.md")]

// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as teleparse;

// re-export user-facing proc-macros
pub use teleparse_macros::derive_lexicon;

/// prelude for all traits and common traits when working with this library
pub mod prelude {
    pub use crate::derive_lexicon;

    // traits
    pub use crate::ToSpan as _;
    pub use crate::Lexicon as _;
    pub use crate::Lexer as _;
    // pub use crate::SyntaxTree as _;

    // pub use crate::tp;
    // util
    pub use crate::{
        Pos,Span
    };

    // pub use crate::{derive_root, assert_ll1};

}

pub mod lex;
#[doc(inline)]
pub use lex::{ToSpan, Lexer, Lexicon};
#[doc(inline)]
pub use lex::{Pos, Span};

pub mod syntax;
pub use syntax::{AbstractSyntaxTree, AbstractSyntaxRoot};

pub mod parser;
pub use parser::Parser;

// pub mod tp;



#[cfg(test)]
pub(crate) mod test;

/// Error when constructing the grammar (i.e. not actually parsing yet).
#[derive(Debug, Clone, thiserror::Error)]
pub enum GrammarError {
    #[error("Cannot construct lexer: {0}")]
    LexerError(#[from] lex::Error),
    #[error("Left recursion detected in the grammar! Stack: {0}")]
    LeftRecursion(String),
    #[error("The non-terminal `{0}` has a FIRST/FIRST conflict producing `{1}`/`{2}`. The conflicting terminals are: {3}")]
    FirstFirstConflict(String, String, String, String),
    #[error("The non-terminal `{0}` has a FIRST/FOLLOW conflict producing `{1}`/`{2}`. The conflicting terminals are: {3}")]
    FirstFollowStringConflict(String, String, String, String),
    #[error("The non-terminal `{0}` has conflict in its FIRST and FOLLOW sets. The conflicting terminals are: {1}")]
    FirstFollowConflict(String, String),
}
