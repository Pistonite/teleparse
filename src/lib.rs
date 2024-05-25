
// So macro works in tests, see 
// https://github.com/bkchr/proc-macro-crate/issues/14
extern crate self as llnparse;

// re-export all proc-macros
pub use llnparse_macros::*;

mod token;
pub use token::*;

#[cfg(not(feature = "arc"))]
pub type Rc<T> = std::rc::Rc<T>;
#[cfg(feature = "arc")]
pub type Rc<T> = std::sync::Arc<T>;


