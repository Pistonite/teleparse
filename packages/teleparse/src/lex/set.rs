use num::Zero;

use super::Lexicon;

/// Efficiently stores multiple token types as a bit set
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Set<L: Lexicon>(L::Bit);

impl<L: Lexicon> Default for Set<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: Lexicon> std::fmt::Debug for Set<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut set = f.debug_set();
        for ty in *self {
            set.entry(&ty);
        }
        set.finish()
    }
}

impl<L: Lexicon> Set<L> {
    /// Create a new empty set
    pub fn new() -> Self {
        Self(L::Bit::zero())
    }

    pub fn from_bits(bits: L::Bit) -> Self {
        Self(bits)
    }

    pub fn to_bits(&self) -> L::Bit {
        self.0
    }

    /// Insert a token type into the set
    pub fn insert(&mut self, ty: L) {
        self.0 = self.0 | ty.to_bit();
    }

    /// Remove a token type from the set
    pub fn remove(&mut self, ty: L) {
        self.0 = self.0 & !ty.to_bit();
    }

    /// Check if the set contains a token type
    pub fn contains(&self, ty: L) -> bool {
        self.0 & ty.to_bit() != L::Bit::zero()
    }

    pub fn contains_all(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Check if the set is empty
    pub fn is_empty(&self) -> bool {
        self.0 == L::Bit::zero()
    }

    pub fn union(&self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl<L: Lexicon> IntoIterator for Set<L> {
    type Item = L;
    type IntoIter = Iter<L>;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

// there can be 128 token types max (0-127) to be represented as a bit set
// so we can use a usize greater than that to represent the start state
// of the iterator
const START: usize = 129;

/// Iterator over token types in a set
pub struct Iter<L: Lexicon> {
    set: Set<L>,
    cur: usize,
}

impl<L: Lexicon> Iter<L> {
    pub fn new(set: Set<L>) -> Self {
        Self { set, cur: START }
    }
}

impl<L: Lexicon> Iterator for Iter<L> {
    type Item = L;
    fn next(&mut self) -> Option<Self::Item> {
        // advance
        let mut next = match self.cur {
            START => L::first(),
            cur => L::from_id_unchecked(cur).next()?,
        };
        self.cur = next.id();
        // skip empty slots
        while !self.set.contains(next) {
            next = next.next()?;
            self.cur = next.id();
        }
        Some(next)
    }
}

/// Macro to create a token set from a list of token types
///
/// ```rust
/// use teleparse::lex::Set;
/// use teleparse::{token_set, derive_lexicon};
///
/// #[derive_lexicon]
/// pub enum T {
///    A, B, C
/// }
///
/// let set = token_set!(T { A | C });
/// let mut expected = Set::new();
/// expected.insert(T::A);
/// expected.insert(T::C);
///
/// assert_eq!(set, expected);
/// ```
///
/// use `token_set!()` to create an empty set or `token_set!(T)` to specify the token type
/// for the empty set explicitly
#[macro_export]
macro_rules! token_set {
    () => {
        $crate::lex::Set::new()
    };
    ($L:ty) => {
        $crate::lex::Set::<$L>::new()
    };
    ($L:ty { $($ty:ident)|* }) => { {
        use $crate::lex::Lexicon;
        let b = $( <$L>::$ty.to_bit() )|*;
        $crate::lex::Set::from_bits(b)
    } }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::TestTokenType as T;
    #[test]
    fn new() {
        let set = Set::<T>::new();
        assert!(set.is_empty());
    }

    #[test]
    fn insert() {
        let mut set = Set::<T>::new();
        set.insert(T::A);
        assert!(set.contains(T::A));
        assert!(!set.is_empty());
        assert_eq!(set, token_set!(T { A }));
    }

    #[test]
    fn remove() {
        let mut set = token_set!(T { A | B | C });
        set.remove(T::B);
        assert!(!set.contains(T::B));
        assert_eq!(set, token_set!(T { A | C }));
    }

    #[test]
    fn iter_empty() {
        let set = token_set!(T);
        let mut iter = set.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_single() {
        let set = token_set!(T { A });
        let mut iter = set.into_iter();
        assert_eq!(iter.next(), Some(T::A));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_continous() {
        let set = token_set!(T { A | B | C });
        let mut iter = set.into_iter();
        assert_eq!(iter.next(), Some(T::A));
        assert_eq!(iter.next(), Some(T::B));
        assert_eq!(iter.next(), Some(T::C));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_skip() {
        let set = token_set!(T { A | C });
        let mut iter = set.into_iter();
        assert_eq!(iter.next(), Some(T::A));
        assert_eq!(iter.next(), Some(T::C));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_skip_many() {
        let set = token_set!(T { A | C | D | G });
        let mut iter = set.into_iter();
        assert_eq!(iter.next(), Some(T::A));
        assert_eq!(iter.next(), Some(T::C));
        assert_eq!(iter.next(), Some(T::D));
        assert_eq!(iter.next(), Some(T::G));
        assert_eq!(iter.next(), None);
    }
}
