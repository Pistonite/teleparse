
// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as llnparse;

// re-export all proc-macros
pub use llnparse_macros::*;

/// prelude for all traits and common includes
pub mod prelude {
    pub use crate::{
        llnparse_derive,
        };
    pub use crate::TokenType as _;

    pub use crate::LexerState;
}

/// dependency re-exports for use inside macros
pub mod dep {
    pub use regex::Regex;
    pub use lazy_static::lazy_static;
}

mod token;
pub use token::*;
mod lexer;
pub use lexer::*;

mod syntax_tree;

#[cfg(not(feature = "arc"))]
pub type Rc<T> = std::rc::Rc<T>;
#[cfg(feature = "arc")]
pub type Rc<T> = std::sync::Arc<T>;


