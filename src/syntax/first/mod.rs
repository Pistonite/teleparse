use crate::Lexicon;
use std::any::TypeId;
use std::collections::BTreeMap;

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
}

// used for implementing metadata debug
#[doc(hidden)]
pub struct DebugFirst<'a, 'b, L: Lexicon>(pub &'a First<L>, pub &'b BTreeMap<TypeId, String>);

impl<L: Lexicon> std::fmt::Debug for DebugFirst<'_, '_, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("First");

        for (ty, first_set) in &self.0.map {
            let name = self.1.get(ty).map(|x| x.as_str()).unwrap_or("<unknown>");
            fmt.field(name, first_set);
        }

        fmt.finish()
    }
}

/// Macro for creating [`FirstSet`] from a list of terminals
///
/// See [`terminal_set`](crate::terminal_set) for more information.
#[macro_export]
macro_rules! first_set {
    ($($x:tt)*) => {
        $crate::syntax::FirstSet::from($crate::terminal_set!($($x)*))
    };
}
