
// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as llnparse;

// re-export user-facing proc-macros
pub use teleparse_macros::teleparse_derive;

/// prelude for all traits and common includes
pub mod prelude {
    pub use crate::teleparse_derive;
    pub use crate::LexerState;

    // traits
    pub use crate::TokenType as _;
    pub use crate::Lexer as _;
    pub use crate::SyntaxTree as _;
    pub use crate::SyntaxTreeNoCtx as _;

    // util
    pub use crate::Pos;
    pub use crate::Span;
    pub use crate::imp::{
        node::Node,
    };

}

/// dependency re-exports for use inside macros
pub mod dep {
    pub use regex::Regex;
}

pub mod imp;

mod token;
pub use token::*;
mod lexer;
pub use lexer::*;
mod parser;
pub use parser::*;
mod syntax_tree;
pub use syntax_tree::*;
mod syntax_error;
pub use syntax_error::*;

#[cfg(not(feature = "arc"))]
pub type Rc<T> = std::rc::Rc<T>;
#[cfg(feature = "arc")]
pub type Rc<T> = std::sync::Arc<T>;


