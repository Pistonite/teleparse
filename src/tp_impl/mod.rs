//! Implementation for standard library types and utility types
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::marker::PhantomData;

use deref_derive::{Deref, DerefMut};

use crate::parser::ParseTree;
use crate::syntax::{self, First, FirstBuilder, FirstRel, Follow, FollowBuilder, FollowRel, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, Parser, Span, ToSpan};

mod ast_node;
pub use ast_node::Node;
mod ast_one_or_more;
pub use ast_one_or_more::OneOrMore;
mod ast_option;
use ast_option::OptionAST;
mod ast_passthrough;


pub mod boxed;
// pub mod blanket;
pub mod iter;
pub mod option;
pub mod string;
pub mod tuple;


