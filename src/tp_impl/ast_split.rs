
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::syntax::{First, FirstBuilder, Follow, FollowBuilder, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, Parser, Span, ToSpan};

use super::OptionAST;

// Split<T> => T Option<OneOrMore<<(P, T)>>>
#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct SplitAST<T: AbstractSyntaxTree, P: AbstractSyntaxTree>(pub Vec<T>, pub Vec<P>);
