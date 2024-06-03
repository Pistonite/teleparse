#![doc = include_str!("./README.md")]

use std::any::TypeId;
use std::collections::BTreeMap;
use crate::Lexicon;

mod builder;
pub use builder::*;
mod set;
pub use set::*;

/// The FIRST function for a grammar.
///
/// See [module-level documentation](self) for more information.
///
/// # Implementation note
/// This type is immutable by-design. The FIRST function is built
/// using a [`FirstBuilder`] by traversing the AST. Once built, it
/// should not be mutated when parsing.
#[derive(Debug, Default)]
pub struct First<L: Lexicon> {
    /// The first set of each AST type
    map: BTreeMap<TypeId, FirstSet<L>>,
    /// The empty first set, used as a fallback instead of panicking
    empty: FirstSet<L>,
}

impl<L: Lexicon> First<L> {
    /// Create a FIRST function backed by the table
    pub fn new(map: BTreeMap<TypeId, FirstSet<L>>) -> Self {
        Self {
            map,
            empty: FirstSet::default(),
        }
    }
    /// Get the [FIRST set](`FirstSet`) for a type
    #[inline]
    pub fn get(&self, ty: &TypeId) -> &FirstSet<L> {
        self.map.get(ty).unwrap_or(&self.empty)
    }

    // /// Check if FIRST(A) contains epsilon and FIRST(B) intersects with FIRST(A)
    // #[must_use]
    // pub fn has_collision(&self, a: &TypeId, b: &TypeId) -> bool {
    //     let first_a = self.get(a);
    //     if !first_a.contains_epsilon() {
    //         return false;
    //     }
    //     first_a.intersects(self.get(b))
    // }
}
