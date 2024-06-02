use std::fmt::{self, Debug, Formatter};
use std::collections::BTreeSet;

use derivative::Derivative;

/// A set of literal constants
#[derive(Derivative, Clone, PartialEq)]
#[derivative(Default(new="true"))]
pub enum LitSet {
    /// A finite set
    #[derivative(Default)]
    Match(BTreeSet<&'static str>),
    /// Universal set
    Any,
}

impl Debug for LitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Match(set) => write!(f, "{:?}", set),
            Self::Any => write!(f, "*"),
        }
    }
}

impl<T> From<T> for LitSet
where
    T: IntoIterator<Item = &'static str>,
{
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

    /// Insert a literal constants into the set
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

    /// Union `self` with the universal set `U` to make self the universal set.
    ///
    /// Returns if `self` is changed (i.e. not already equal to `U`)
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
                *s= Self::Any;
                !is_self_any
            }
        }
    }
    
    /// Test if this set intersects with another set
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                set.iter().any(|lit| {
                    other_set.contains(lit)
                })
            }
            (Self::Any, _) | (_, Self::Any) => true,
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
    fn intersect_empty_universe() {
        let set1 = LitSet::new();
        let set2 = LitSet::universe();
        assert!(!set1.intersects(&set1));
        assert!(set1.intersects(&set2));
        assert!(set2.intersects(&set1));
        assert!(set2.intersects(&set2));
    }

    #[test]
    fn intersect_universe() {
        let u = LitSet::universe();
        let set = LitSet::from(["a", "b"]);

        assert!(u.intersects(&set));
        assert!(set.intersects(&u));
    }

    #[test]
    fn intersect_disjoint() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["c", "d"]);
        assert!(!set1.intersects(&set2));
        assert!(!set2.intersects(&set1));
    }

    #[test]
    fn intersect_subset() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["a"]);
        assert!(set1.intersects(&set2));
        assert!(set2.intersects(&set1));
    }

    #[test]
    fn intersect_empty() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::new();
        assert!(!set1.intersects(&set2));
        assert!(!set2.intersects(&set1));
    }
}
