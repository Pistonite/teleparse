
// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as teleparse;

// re-export user-facing proc-macros
pub use teleparse_macros::teleparse_derive;

/// prelude for all traits and common includes
pub mod prelude {
    pub use crate::teleparse_derive;

    // traits
    pub use crate::TokenType as _;
    pub use crate::Lexer as _;
    pub use crate::SyntaxTree as _;
    pub use crate::SyntaxTreeNoCtx as _;

    // util
    pub use crate::{
        Pos,Span,Token
    };

}

pub mod imp;

pub mod token;
pub use token::{Pos, Span, Token, TokenType, TokenStorage};

// lexer re-exports
pub mod lexer;
pub use lexer::Lexer;

mod parser;
pub use parser::*;
mod syntax_tree;
pub use syntax_tree::*;
mod syntax_error;
pub use syntax_error::*;

