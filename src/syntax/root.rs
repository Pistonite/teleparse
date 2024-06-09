
use std::any::TypeId;
use std::collections::BTreeMap;

use crate::{GrammarError, Lexicon};

use super::{AbstractSyntaxTree, DebugFirst, DebugFollow, DebugJump, First, Follow, Jump};

/// The root of the Abstract Syntax Tree (AST) for a grammar
///
/// Deriving this trait provides static storage of the metadata of the grammar such as 
/// the FIRST and FOLLOW functions.
pub trait AbstractSyntaxRoot: AbstractSyntaxTree {
    /// Get the static metadata
    fn metadata() -> &'static Result<Metadata<Self::L>, GrammarError>;

    #[cfg(test)]
    fn assert_ll1() {
        if let Err(e) = Self::metadata() {
            assert!(false, "{} is not LL(1): {}", Self::debug(), e);
        }
    }
}

pub struct Metadata<L: Lexicon>{
    pub names: BTreeMap<TypeId, String>,
    pub first: First<L>,
    pub follow: Follow<L>,
    pub jump: Jump<L>,
}

impl<L: Lexicon> std::fmt::Debug for Metadata<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("first", &DebugFirst(&self.first, &self.names))
            .field("follow", &DebugFollow(&self.follow, &self.names))
            .field("jump", &DebugJump(&self.jump, &self.names))
            .finish()
    }
}
