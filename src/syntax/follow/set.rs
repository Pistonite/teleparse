use std::collections::BTreeSet;

use derivative::Derivative;

use crate::lex::TokenSrc;
use crate::syntax::{FirstSet, TerminalSet};
use crate::Lexicon;

/// Implementation of the output of the FOLLOW function
///
/// See [module-level documentation](super) for more information.
#[derive(Derivative, PartialEq, Clone)]
#[derivative(Default(new = "true", bound = ""))]
pub struct FollowSet<L: Lexicon>(TerminalSet<L>);

impl<L: Lexicon> std::fmt::Debug for FollowSet<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<L: Lexicon> FollowSet<L> {
    /// Insert EOF into the set. 
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn insert_eof(&mut self) {
        self.0.insert_e();
    }

    /// Check if the set contains EOF
    #[inline]
    pub fn contains_eof(&self) -> bool {
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

    /// Union with a FIRST set. (`epsilon` will not be included, since FOLLOW sets do not contain
    /// them)
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn union_first(&mut self, other: &FirstSet<L>) -> bool {
        self.0.union(other.as_terminal_set(), false)
    }


    /// Union with another FOLLOW set
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn union_follow(&mut self, other: &Self) -> bool {
        self.0.union(&other.0, true)
    }

    /// Check if the FOLLOW set intersects with a FIRST set
    ///
    /// Note that `epsilon` in the FIRST set is not considered,
    /// since FOLLOW sets do not contain them.
    #[inline]
    pub fn intersects_first(&self, other: &FirstSet<L>) -> bool {
        self.0.intersects(other.as_terminal_set(), false)
    }

    /// See [`TerminalSet::intersection_repr`]
    #[inline]
    pub fn intersection_repr_first(&self, other: &FirstSet<L>) -> BTreeSet<String> {
        self.0.intersection_repr(other.as_terminal_set(), false)
    }
}
