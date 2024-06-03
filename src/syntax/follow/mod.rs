#![doc = include_str!("./README.md")]

use std::any::TypeId;
use std::collections::BTreeMap;

use crate::Lexicon;

mod builder;
pub use builder::*;
mod set;
pub use set::*;

/// The FOLLOW function for a grammar.
///
/// See [module-level documentation](self) for more information.
///
/// # Implementation note
/// This type is immutable by-design. The FIRST function is built
/// using a [`FollowBuilder`] by traversing the AST. Once built, it
/// should not be mutated when parsing.
pub struct Follow<L: Lexicon> {
    map: BTreeMap<TypeId, FollowSet<L>>,
    empty: FollowSet<L>,
}

impl<L: Lexicon> Follow<L> {
    /// Create a FOLLOW function backed by the table
    pub fn new(map: BTreeMap<TypeId, FollowSet<L>>) -> Self {
        Self {
            map,
            empty: FollowSet::default(),
        }
    }
    /// Get the [FOLLOW set](`FollowSet`) for a type
    #[inline]
    pub fn get(&self, t: &TypeId) -> &FollowSet<L> {
        self.map.get(t).unwrap_or(&self.empty)
    }
}


