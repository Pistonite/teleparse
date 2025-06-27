use std::collections::BTreeSet;

use derivative::Derivative;

/// A set of literal constants
#[derive(Derivative, Clone, PartialEq)]
#[derivative(Default(new = "true"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "set"))]
pub enum LitSet {
    /// A finite set
    #[derivative(Default)]
    Match(BTreeSet<&'static str>),
    /// Universal set
    Any,
}

impl std::fmt::Debug for LitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Match(set) => write!(f, "{set:?}"),
            Self::Any => write!(f, "*"),
        }
    }
}

impl<T: IntoIterator<Item = &'static str>> From<T> for LitSet {
    #[inline]
    fn from(set: T) -> Self {
        Self::Match(set.into_iter().collect())
    }
}

impl LitSet {
    /// Create the universal set `U` that contains all literals
    #[inline]
    pub fn universe() -> Self {
        Self::Any
    }

    /// Check if this set is the universal set `U`
    #[inline]
    pub fn is_universe(&self) -> bool {
        matches!(self, Self::Any)
    }

    /// Clear the set
    #[inline]
    pub fn clear(&mut self) {
        match self {
            Self::Match(set) => set.clear(),
            Self::Any => {}
        }
    }

    /// Check if the set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Match(set) => set.is_empty(),
            Self::Any => false,
        }
    }

    /// Insert a literal constants into the set
    ///
    /// Returns if the set is changed (i.e. not already containing the literal)
    #[inline]
    pub fn insert(&mut self, lit: &'static str) -> bool {
        match self {
            Self::Match(set) => set.insert(lit),
            Self::Any => false,
        }
    }

    /// Check if the set contains a literal constant
    #[inline]
    pub fn contains(&self, lit: &str) -> bool {
        match self {
            Self::Match(set) => set.contains(lit),
            Self::Any => true,
        }
    }

    /// Union this set with the universal set `U` to make self the universal set.
    ///
    /// Returns if the set is changed (i.e. not already equal to `U`)
    #[inline]
    pub fn union_universe(&mut self) -> bool {
        match self {
            Self::Match(_) => {
                *self = Self::Any;
                true
            }
            Self::Any => false,
        }
    }

    /// Union other into self (self = self U other). Return if self is changed
    pub fn union(&mut self, other: &Self) -> bool {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                let old_size = set.len();
                set.extend(other_set.iter());
                old_size != set.len()
            }
            (s, _) => {
                let is_self_any = matches!(s, Self::Any);
                *s = Self::Any;
                !is_self_any
            }
        }
    }

    /// Test if this set intersects with another set
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                set.intersection(other_set).next().is_some()
            }
            (Self::Any, s) | (s, Self::Any) => !s.is_empty(),
        }
    }

    /// Create a new set containing the intersection of this set and another set
    pub fn intersection(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                Self::Match(set.intersection(other_set).copied().collect())
            }
            (Self::Any, Self::Any) => Self::Any,
            (Self::Any, s) | (s, Self::Any) => s.clone(),
        }
    }

    /// Get an iterator over the literals in the set
    ///
    /// If the set is the universal set `U`, `None` is returned since the set has infinite size
    pub fn iter(&self) -> Option<impl Iterator<Item = &&'static str>> {
        match self {
            Self::Match(set) => Some(set.iter()),
            Self::Any => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn universe_contains_everything() {
        let set = LitSet::universe();
        assert!(set.contains(""));
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(set.contains("c"));
    }

    #[test]
    fn contains_static() {
        let mut set = LitSet::new();
        set.insert("a");
        assert!(set.contains("a"));
    }

    #[test]
    fn contains_static_not() {
        let mut set = LitSet::new();
        set.insert("a");
        assert!(!set.contains("b"));
    }

    #[test]
    fn contains_dynamic() {
        let a = "a";
        let b = "a".to_string();
        assert!(!std::ptr::eq(a, b.as_str()));
        let mut set = LitSet::new();
        set.insert(a);
        assert!(set.contains(&b));
    }

    #[test]
    fn union_universe_from_empty() {
        let mut set = LitSet::new();
        assert!(set.union_universe());
        assert_eq!(set, LitSet::universe());
    }

    #[test]
    fn union_universe_from_some() {
        let mut set = LitSet::from(["a"]);
        assert!(set.union_universe());
        assert_eq!(set, LitSet::universe());
    }

    #[test]
    fn union_universe_from_universe() {
        let mut set = LitSet::universe();
        assert!(!set.union_universe());
        assert_eq!(set, LitSet::universe());
    }

    #[test]
    fn union() {
        let mut set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["b", "c"]);
        let expected = LitSet::from(["a", "b", "c"]);
        assert!(set1.union(&set2));
        assert_eq!(set1, expected);
    }

    #[test]
    fn union_no_change_empty() {
        let mut set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::new();
        assert!(!set1.union(&set2));
    }

    #[test]
    fn union_no_change_subset() {
        let mut set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["a"]);
        assert!(!set1.union(&set2));
    }

    #[test]
    fn union_no_change_equal() {
        let mut set1 = LitSet::from(["a", "b"]);
        let set2 = set1.clone();
        assert!(!set1.union(&set2));
    }

    #[test]
    fn union_disjoint() {
        let mut set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["d", "c"]);
        let expected = LitSet::from(["a", "b", "c", "d"]);
        assert!(set1.union(&set2));
        assert_eq!(set1, expected);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)] //it's clearer
    fn intersect_empty_universe() {
        let set1 = LitSet::new();
        let set2 = LitSet::universe();
        assert_eq!(set1.intersects(&set1), false);
        assert_eq!(set1.intersects(&set2), false);
        assert_eq!(set2.intersects(&set1), false);
        assert_eq!(set2.intersects(&set2), true);
    }

    #[test]
    fn intersect_universe() {
        let u = LitSet::universe();
        let set = LitSet::from(["a", "b"]);

        assert!(u.intersects(&set));
        assert!(set.intersects(&u));
        assert_eq!(u.intersection(&set), set);
        assert_eq!(set.intersection(&u), set);
    }

    #[test]
    fn intersect_disjoint() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["c", "d"]);
        assert!(!set1.intersects(&set2));
        assert!(!set2.intersects(&set1));
        assert_eq!(set2.intersection(&set1), LitSet::new());
        assert_eq!(set1.intersection(&set2), LitSet::new());
    }

    #[test]
    fn intersect_subset() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["a"]);
        assert!(set1.intersects(&set2));
        assert!(set2.intersects(&set1));
        assert_eq!(set2.intersection(&set1), set2);
        assert_eq!(set1.intersection(&set2), set2);
    }

    #[test]
    fn intersect_empty() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::new();
        assert!(!set1.intersects(&set2));
        assert!(!set2.intersects(&set1));
        assert_eq!(set2.intersection(&set1), set2);
        assert_eq!(set1.intersection(&set2), set2);
    }

    #[test]
    fn intersection() {
        let set1 = LitSet::from(["a", "b", "c"]);
        let set2 = LitSet::from(["a", "c", "d"]);
        let expected = LitSet::from(["a", "c"]);
        assert_eq!(set1.intersection(&set2), expected);
        assert_eq!(set2.intersection(&set1), expected);
    }
}
