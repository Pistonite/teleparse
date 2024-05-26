
// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as llnparse;

// re-export all proc-macros
pub use llnparse_macros::*;

// prelude for all traits and common includes
pub mod prelude {
    pub use crate::TokenType as _;
}

mod token;
pub use token::*;
mod lexer;

mod syntax_tree;

#[cfg(not(feature = "arc"))]
pub type Rc<T> = std::rc::Rc<T>;
#[cfg(feature = "arc")]
pub type Rc<T> = std::sync::Arc<T>;


