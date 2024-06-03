use std::collections::BTreeSet;

use derivative::Derivative;

use crate::lex::TokenSrc;
use crate::syntax::{JumpTable, TerminalSet};
use crate::Lexicon;

/// Implementation of the output of the FIRST function
///
/// See [module-level documentation](super) for more information.
#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new="true", bound=""))]
pub struct FirstSet<L: Lexicon>(TerminalSet<L>);

impl<L: Lexicon> FirstSet<L> {
    /// Create a new FIRST set with the term `(ty, lit)` in it. None indicates any literal.
    #[inline]
    pub fn one(ty: L, lit: Option<&'static str>) -> Self {
        let mut set = Self::default();
        set.insert(ty, lit);
        set
    }
    /// Clear the set
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Insert epsilon into the set. 
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn insert_epsilon(&mut self) -> bool {
        self.0.insert_e()
    }
    
    /// Check if the set contains epsilon
    #[inline]
    pub fn contains_epsilon(&self) -> bool {
        self.0.contains_e()
    }

    /// Insert the term `(ty, lit)` into the set. None indicates any literal.
    #[inline]
    pub fn insert(&mut self, ty: L, lit: Option<&'static str>) -> bool {
        self.0.insert(ty, lit)
    }

    /// Check if the set contains the term `(ty, lit)`
    #[inline]
    pub fn contains<'s>(&self, token: Option<TokenSrc<'s, L>>) -> bool {
        self.0.contains(token)
    }

    /// Union with another FIRST set without considering epsilon
    ///
    /// Effectively:
    /// ```test
    /// Self = Self U (Other - { epsilon })
    /// ```
    ///
    /// Returns if self is changed
    #[inline]
    pub fn union_minus_epsilon(&mut self, other: &Self) -> bool {
        self.0.union(&other.0, false)
    }

    /// Test if the set intersects with another FIRST set, including epsilon
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        self.0.intersects(&other.0, true)
    }

    /// Test if the set intersects `Other - { epsilon }`
    #[inline]
    pub fn intersects_minus_epsilon(&self, other: &Self) -> bool {
        self.0.intersects(&other.0, false)
    }

    /// See [`TerminalSet::intersection_repr`]
    #[inline]
    pub fn intersection_repr_minus_epsilon(&self, other: &Self) -> BTreeSet<String> {
        self.0.intersection_repr(&other.0, false)
    }

    /// Add (ty, lit) -> value to the jump table for every terminal in the set
    pub fn add_to_jump_table(&self, table: &mut JumpTable<L>, value: usize) {
        if self.contains_epsilon() {
            table.register_epsilon(value);
        }
        for (ty, set) in self.0.map.iter_zip() {
            match set.iter() {
                None => table.register(value, ty),
                Some(lits) => {
                    for lit in lits {
                        table.register_lit(value, ty, lit);
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn as_terminal_set(&self) -> &TerminalSet<L> {
        &self.0
    }
}


