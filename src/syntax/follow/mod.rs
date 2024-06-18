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
#[derive(Debug, Default)]
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

// used for implementing metadata debug
#[doc(hidden)]
pub struct DebugFollow<'a, 'b, L: Lexicon>(pub &'a Follow<L>, pub &'b BTreeMap<TypeId, String>);

impl<'a, 'b, L: Lexicon> std::fmt::Debug for DebugFollow<'a, 'b, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Follow");

        for (ty, follow_set) in &self.0.map {
            let name = self.1.get(ty).map(|x| x.as_str()).unwrap_or("<unknown>");
            fmt.field(name, follow_set);
        }

        fmt.finish()
    }
}

/// Macro for creating [`FollowSet`] from a list of terminals
///
/// See [`terminal_set`](crate::terminal_set) for more information.
#[macro_export]
macro_rules! follow_set {
    ($($x:tt)*) => {
        $crate::syntax::FollowSet::from($crate::terminal_set!($($x)*));
    };
}
