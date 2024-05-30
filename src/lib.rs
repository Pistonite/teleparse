
// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as teleparse;

// re-export user-facing proc-macros
pub use teleparse_macros::{teleparse_derive, ToSpan};

/// prelude for all traits and common includes when working with this library
pub mod prelude {
    pub use crate::teleparse_derive;

    // traits
    pub use crate::ToSpan as _;
    pub use crate::TokenType as _;
    pub use crate::TokenTypeNoCtx as _;
    pub use crate::Lexer as _;
    pub use crate::SyntaxTree as _;
    pub use crate::Root as _;
    pub use crate::RootNoCtx as _;

    // util
    pub use crate::{
        Pos,Span,Token
    };

}

pub mod node;

pub mod token;
pub use token::{Pos, Span, ToSpan, Token, TokenType, TokenTypeNoCtx, TokenStorage};

// lexer re-exports
pub mod lexer;
pub use lexer::Lexer;

pub mod parser;
pub use parser::{Parser, ParserIter};
mod syntax_tree;
pub use syntax_tree::*;
mod syntax_error;
pub use syntax_error::*;


pub mod table;
