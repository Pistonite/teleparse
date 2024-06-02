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

    #[inline]
    pub fn is_universe(&self) -> bool {
        matches!(self, Self::Any)
    }

    #[inline]
    pub fn clear(&mut self) {
        match self {
            Self::Match(set) => set.clear(),
            Self::Any => {},
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Match(set) => set.is_empty(),
            Self::Any => false,
        }
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
                set.intersection(other_set).next().is_some()
            }
            (Self::Any, s) | (s, Self::Any) => !s.is_empty(),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                Self::Match(set.intersection(other_set).copied().collect())
            }
            (Self::Any, Self::Any)  => Self::Any,
            (Self::Any, s) | (s, Self::Any) => {
                s.clone()
            }
        }
    }

    pub fn iter(&self) -> Option<impl Iterator<Item = &&'static str>> {
        match self {
            Self::Match(set) => Some(set.iter()),
            Self::Any => None,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Intersects {
    None,
    First(&'static str),
    Universe
}

pub enum Intersection<'a> {
    Single(std::collections::btree_set::Iter<'a, &'static str>),
    Both(std::collections::btree_set::Intersection<'a, &'static str>),
}

impl<'a> Iterator for Intersection<'a> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self {
            Self::Single(iter) => iter.next(),
            Self::Both(iter) => iter.next(),
        };
        next.map(|x|*x)
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
        assert_eq!(set1.intersects(&set1), false);
        assert_eq!(set1.intersects(&set2), false);
        assert_eq!(set2.intersects(&set1), false);
        assert_eq!(set2.intersects(&set2), true);
    }

    #[test]
    fn intersect_universe() {
        let u = LitSet::universe();
        let set = LitSet::from(["a", "b"]);

        assert_eq!(u.intersects(&set), true);
        assert_eq!(set.intersects(&u), true);
        assert_eq!(u.intersection(&set), set);
        assert_eq!(set.intersection(&u), set);
    }

    #[test]
    fn intersect_disjoint() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["c", "d"]);
        assert_eq!(set1.intersects(&set2), false);
        assert_eq!(set2.intersects(&set1), false);
        assert_eq!(set2.intersection(&set1), LitSet::new());
        assert_eq!(set1.intersection(&set2), LitSet::new());
    }

    #[test]
    fn intersect_subset() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::from(["a"]);
        assert_eq!(set1.intersects(&set2), true);
        assert_eq!(set2.intersects(&set1), true);
        assert_eq!(set2.intersection(&set1), set2);
        assert_eq!(set1.intersection(&set2), set2);
    }

    #[test]
    fn intersect_empty() {
        let set1 = LitSet::from(["a", "b"]);
        let set2 = LitSet::new();
        assert_eq!(set1.intersects(&set2), false);
        assert_eq!(set2.intersects(&set1), false);
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
