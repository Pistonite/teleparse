use std::collections::BTreeSet;

use derivative::Derivative;

use crate::lex::{Map, TokenSrc};

use crate::Lexicon;

use super::LitSet;


/// A set of terminal symbols, and possibly a special `empty` symbol
///
/// The implementation should be fast to look up if a token (T, src) is in the set,
/// which makes it suitable as the implementation for [FIRST](super::first) and
/// [FOLLOW](super::follow) sets.
#[derive(Derivative, Debug, Clone)]
#[derivative(Default(new="true", bound=""))]
pub struct TerminalSet<L: Lexicon> {
    /// Maps token to the set of literals to form the terminal set
    pub map: Map<L, LitSet>,
    /// Whether the set contains the empty symbol
    e: bool,
}

impl<L: Lexicon> TerminalSet<L> {
    /// Clear the set
    #[inline]
    pub fn clear(&mut self) {
        self.e = false;
        for set in self.map.iter_mut() {
            set.clear();
        }
    }
    /// Insert the empty symbol into the set. 
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn insert_e(&mut self) -> bool {
        let changed = !self.e;
        self.e = true;
        changed
    }
    
    /// Check if the set contains the empty symbol
    #[inline]
    pub fn contains_e(&self) -> bool {
        self.e
    }

    /// Insert the terminal `(ty, lit)` into the set. None indicates any literal.
    pub fn insert(&mut self, ty: L, lit: Option<&'static str>) -> bool {
        match lit {
            Some(lit) => {
                self.map.get_mut(ty).insert(lit)
            },
            None => {
                self.map.get_mut(ty).union_universe()
            }
        }
    }

    /// Check if the set contains the terminal `(ty, lit)`
    pub fn contains<'s>(&self, token: Option<TokenSrc<'s, L>>) -> bool {
        match token {
            None => self.e,
            Some(token) => {
                self.map.get(token.ty).contains(&token.src)
            }
        }
    }

    /// Union with another [`TerminalSet`] 
    ///
    /// Effectively:
    /// ```text
    /// Self = Self U Other
    /// ```
    ///
    /// Returns if self is changed
    /// ## Excluding empty symbol
    /// if `minus_e` is true, the empty symbol is excluded from `other`
    /// when unioning. 
    /// Effectively: 
    /// ```text
    /// Self = Self U (Other - {E})
    /// ```
    /// The other way to interpret this operation is to consider the `empty`
    /// from the other set as a different symbol from the `empty` in this set,
    /// and is not allowed in this set.
    pub fn union(&mut self, other: &Self, minus_e: bool) -> bool {
        let mut changed = if minus_e {
            false
        } else {
            if !self.e && other.e {
                self.insert_e();
                true
            } else {
                false
            }
        };
        for (set, other_set) in self.map.iter_mut().zip(other.map.iter()) {
            let next_changed = set.union(other_set);
            changed |= next_changed;
        }
        changed
    }

    /// Test if two [`TerminalSet`]s have non-empty intersection
    ///
    /// ## Excluding empty symbol
    /// if `include_e` is false, the empty symbol is excluded from `other`
    /// when testing
    ///
    /// The other way to interpret this operation is to consider the `empty`
    /// from the other set as a different symbol from the `empty` in this set,
    /// in which case it will never appear in the intersection.
    ///
    /// This returns immediately when one terminal is found in both sets
    pub fn intersects(&self, other: &Self, include_e: bool) -> bool {
        if include_e {
            if self.e && other.e {
                return true;
            }
        }
        for (set, other_set) in self.map.iter().zip(other.map.iter()) {
            if set.intersects(other_set) {
                return true;
            }
        }
        false
    }

    /// Compute the intersection of two [`TerminalSet`]s, and return the result as
    /// a set of string representations of the terminals
    ///
    /// ## Representation
    /// If a terminal has a literal value to match, then the string representation
    /// is the literal surrounded by quotes (`"`). If the terminal has no literal,
    /// the name of the token type is returned without quotes.
    ///
    /// The empty symbol is represented as the empty string `""`.
    ///
    /// ## Excluding empty symbol
    /// if `include_e` is false, the empty symbol is excluded from `other`
    /// when testing
    ///
    /// The other way to interpret this operation is to consider the `empty`
    /// from the other set as a different symbol from the `empty` in this set,
    /// in which case it will never appear in the intersection.
    pub fn intersection_repr(&self, other: &Self, include_e: bool) -> BTreeSet<String> {
        let mut terminals = BTreeSet::new();
        if include_e {
            if self.e && other.e {
                terminals.insert("".to_string());
            }
        }
        for ((ty, set), other_set) in self.map.iter_zip().zip(other.map.iter()) {
            let intersection = set.intersection(other_set);
            if intersection.is_empty() {
                continue;
            }
            match intersection.iter() {
                Some(lits) => {
                    for lit in lits {
                        terminals.insert(format!("\"{}\"", lit));
                    }
                }
                None => {
                    terminals.insert(format!("{:?}", ty));
                }
            };
        }
        terminals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::TestTokenType as T;

    #[test]
    fn insert_epsilon() {
        let mut set = TerminalSet::<T>::new();
        assert!(!set.contains_e());
        assert!(!set.contains(None));
        assert_eq!(set.insert_e(), true);
        assert!(set.contains_e());
        assert!(set.contains(None));
        assert_eq!(set.insert_e(), false);
        assert!(set.contains_e());
        assert!(set.contains(None));
    }

    #[test]
    fn insert() {
        let mut set = TerminalSet::new();
        assert!(set.insert(T::A, Some("a")));
        assert!(set.contains(Some((T::A, "a").into())));
        assert!(!set.insert(T::A, Some("a")));
        assert!(set.insert(T::A, None));
        assert!(set.contains(Some((T::A, "a").into())));
        assert!(set.contains(Some((T::A, "b").into())));
        assert!(!set.contains(Some((T::B, "a").into())));
        assert!(set.insert(T::B, Some("a")));
        assert!(set.contains(Some((T::B, "a").into())));
        assert!(!set.contains(Some((T::B, "b").into())));
    }

    #[test]
    fn todo() {
        todo!()
    }

}