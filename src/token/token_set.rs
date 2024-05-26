use num::Zero;

use super::TokenType;

/// Stores multiple token types as a bit set
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenTySet<T: TokenType>(T::Repr);

impl<T: TokenType> TokenTySet<T> {
    /// Create a new empty set
    #[inline]
    pub fn new() -> Self {
        Self(T::Repr::zero())
    }

    /// Insert a token type into the set
    #[inline]
    pub fn insert(&mut self, token_type: T) {
        self.0 = self.0 | token_type.to_repr();
    }

    /// Remove a token type from the set
    #[inline]
    pub fn remove(&mut self, token_type: T) {
        self.0 = self.0 & !token_type.to_repr();
    }

    /// Check if the set contains a token type
    #[inline]
    pub fn contains(&self, token_type: T) -> bool {
        self.0 & token_type.to_repr() != T::Repr::zero()
    }

    /// Check if the set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == T::Repr::zero()
    }
}

impl<T: TokenType> IntoIterator for TokenTySet<T> {
    type Item = T;
    type IntoIter = TTSetIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        TTSetIter::from(self.0)
    }
}

pub struct TTSetIter<T: TokenType> {
    set: T::Repr,
    cur: Option<T>,
    done: bool,
}

impl<T: TokenType> TTSetIter<T> {
    pub fn from(repr: T::Repr) -> Self {
        Self {
            set: repr,
            cur: None,
            done: false,
        }
    }
}

impl<T: TokenType> Iterator for TTSetIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut next = match self.cur {
            None => T::first(),
            Some(cur) => match cur.next() { 
                Some(next) => next,
                None => {
                    self.done = true;
                    return None;
                }
            }
        };
        while self.set & next.to_repr() == T::Repr::zero() {
            next = match next.next() {
                Some(next) => next,
                None => {
                    self.done = true;
                    return None;
                }
            };
        }
        self.cur = Some(next);
        Some(next)
    }
}
