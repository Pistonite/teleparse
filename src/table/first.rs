use std::any::TypeId;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{BTreeMap, BTreeSet};

use derivative::Derivative;

use crate::token::TokenSrc;
use crate::{SyntaxTree, TokenType};

use super::LitSet;


pub struct FirstBuilder<T: TokenType> {
    seen: BTreeSet<TypeId>,
    exprs: Vec<FirstExpr<T>>,
}

impl<T: TokenType> FirstBuilder<T> {
    #[inline(always)]
    pub fn build_recursive<ST: SyntaxTree<T=T>>(&mut self) {
        let t = ST::type_id();
        if self.seen.insert(t) {
            ST::build_first(self);
        }
    }

    #[inline]
    pub fn add(&mut self, expr: FirstExpr<T>) {
        self.exprs.push(expr);
    }

    /// Build first table for X -> Y1 | Y2 | ... | Yn
    #[inline]
    pub fn build_for_enum(&mut self, x: TypeId, variants: &[TypeId]) {
        for y in variants {
            self.add(FirstExpr::union_minus_epsilon(x, *y));
            self.add(FirstExpr::if_epsilon_in_all([y], FirstExpr::insert_epsilon(x)));
        }
    }

    /// Build first table for X -> Y1 Y2 ... Yn
    #[inline]
    pub fn build_for_sequence(&mut self, x: TypeId, sequence: &[TypeId]) {
        let mut set = BTreeSet::new();
        for yi in sequence {
            // if Y1 ... Yi-1 can all produce epsilon, add FIRST(Yi) - { epsilon }
            self.add(FirstExpr::if_epsilon_in_all(set.iter(), FirstExpr::union_minus_epsilon(x, *yi)));
            set.insert(*yi);
        }
        // if all Y1 ... Yn can produce epsilon, add epsilon
        self.add(FirstExpr::IfEpsilonInAll(set, Box::new(FirstExpr::insert_epsilon(x))));
    }

    pub fn build(mut self) -> First<T> {
        let mut first = BTreeMap::<TypeId, FirstSet<T>>::new();
        let mut changed = true;

        while changed {
            changed = false;
            for expr in self.exprs.iter_mut() {
                match expr {
                    FirstExpr::Insert(t, insert) => {
                        let set = first.entry(*t).or_default();
                        match insert {
                            FirstInsert::Epsilon => {
                                changed = set.insert_epsilon() || changed;
                            },
                            FirstInsert::Token(token, lit) => {
                                changed = set.insert(*token, *lit) || changed;
                            }
                        }
                        // after something is added to the set, it's always going
                        // to be there, so we can skip further insertions
                        *expr = FirstExpr::Noop;
                    },
                    FirstExpr::UnionMinusEpsilon(a, b) => {
                        let mut first_a = first.remove(a).unwrap_or_default();
                        if let Some(first_b) = first.get(b) {
                            changed = first_a.union_minus_epsilon(first_b) || changed;
                        }
                        first.insert(*a, first_a);
                    }
                    FirstExpr::IfEpsilonInAll(set, inner) => {
                        // keep the sets that don't contain epsilon
                        set.retain(|t| {
                            match first.get(t) {
                                Some(set) => !set.contains_epsilon(),
                                None => true,
                            }
                        });
                        if set.is_empty() {
                            let mut noop = FirstExpr::Noop;
                            std::mem::swap(inner.as_mut(), &mut noop);
                            *expr = noop;
                            changed = true;
                        }
                    }
                    FirstExpr::Noop => {}
                }
            }
        }

        First {
            map: first,
            empty: FirstSet::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct First<T: TokenType> {
    map: BTreeMap<TypeId, FirstSet<T>>,
    empty: FirstSet<T>,
}

impl<T: TokenType> First<T> {
    pub fn get(&self, ty: &TypeId) -> &FirstSet<T> {
        self.map.get(ty).unwrap_or(&self.empty)
    }
    /// Check if FIRST(A) intersects with FIRST(B)
    pub fn has_intersection(&self, a: &TypeId, b: &TypeId) -> bool {
        self.get(a).intersects(self.get(b))
    }

    /// Check if FIRST(A) contains epsilon and FIRST(B) intersects with FIRST(A)
    pub fn has_collision(&self, a: &TypeId, b: &TypeId) -> bool {
        let first_a = self.get(a);
        if !first_a.contains_epsilon() {
            return false;
        }
        first_a.intersects(self.get(b))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FirstExpr<T: TokenType> {
    Noop,
    /// `FIRST(A) = FIRST(A) U Insert`
    Insert(TypeId, FirstInsert<T>),
    /// `FIRST(A) = FIRST(A) U (FIRST(B) - { epsilon })`
    UnionMinusEpsilon(TypeId, TypeId),
    /// If epsilon is in INTERSECTION(FIRST(A)), then execute the inner expression
    IfEpsilonInAll(BTreeSet<TypeId>, Box<FirstExpr<T>>),
}

impl<T: TokenType> FirstExpr<T> {
    #[inline]
    pub fn insert_epsilon(t: TypeId) -> Self {
        Self::Insert(t, FirstInsert::Epsilon)
    }

    #[inline]
    pub fn insert_token(t: TypeId, token: T, lit: Option<&'static str>) -> Self {
        Self::Insert(t, FirstInsert::Token(token, lit))
    }

    #[inline]
    pub fn union_minus_epsilon(a: TypeId, b: TypeId) -> Self {
        Self::UnionMinusEpsilon(a, b)
    }

    #[inline]
    pub fn if_epsilon_in_all<'s, Iter: IntoIterator<Item=&'s TypeId>>(set: Iter, expr: FirstExpr<T>) -> Self {
        let set = set.into_iter().copied().collect::<BTreeSet<_>>();
        if set.is_empty() {
            return expr;
        }
        Self::IfEpsilonInAll(set, Box::new(expr))
    }
}

/// Insert operation for FIRST set. Does not depend on other FIRST sets.
#[derive(Debug, Clone, PartialEq)]
pub enum FirstInsert<T: TokenType> {
    /// Union with `{ (T, lit) }`
    Token(T, Option<&'static str>),
    /// Union with `{ epsilon }`
    Epsilon,
}

/// Implementation of the output of the FIRST function
#[derive(Derivative, Clone)]
#[derivative(Default(new="true"))]
pub struct FirstSet<T: TokenType> {
    has_epsilon: bool,
    /// Maps token ordinal to a set of literals
    array: T::Set,
}

impl<T: TokenType> std::fmt::Debug for FirstSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Set");
        s.field("e", &self.has_epsilon);
        let slice: &[LitSet] = self.array.borrow();
        for (ty, set) in slice.iter().enumerate() {
            s.field(&ty.to_string(), set);
        }
        s.finish()
    }
}

impl<TIter, T: TokenType> From<TIter> for FirstSet<T> 
    where TIter: IntoIterator<Item = (T, Option<&'static str>)>
{
    fn from(value: TIter) -> Self {
        let mut set = Self::new();
        for (ty, lit) in value {
            set.insert(ty, lit);
        }
        set
    }
}

impl<T: TokenType> FirstSet<T> {
    /// Insert epsilon into the set. 
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn insert_epsilon(&mut self) -> bool {
        let changed = !self.has_epsilon;
        self.has_epsilon = true;
        changed
    }
    
    /// Check if the set contains epsilon
    #[inline]
    pub fn contains_epsilon(&self) -> bool {
        self.has_epsilon
    }

    /// Insert the term `(ty, lit)` into the set. None indicates any literal.
    #[inline]
    pub fn insert(&mut self, ty: T, lit: Option<&'static str>) -> bool {
        match lit {
            Some(lit) => {
                self.get_mut(ty).insert(lit)
            },
            None => {
                self.get_mut(ty).union_universe()
            }
        }
    }

    /// Check if the set contains the term `(ty, lit)`
    pub fn contains<'s>(&self, token: Option<TokenSrc<'s, T>>) -> bool {
        match token {
            None => self.has_epsilon,
            Some(token) => {
                self.get(token.ty).contains(&token.src)
            }
        }
    }

    /// Union with another FIRST set `Self = Self U (Other - { epsilon })`
    ///
    /// Returns if self is changed
    #[inline]
    pub fn union_minus_epsilon(&mut self, other: &Self) -> bool {
        union_impl::<T>(&mut self.array, &other.array)
    }

    /// Returns if self intersects with another FIRST set
    pub fn intersects(&self, other: &Self) -> bool {
        if self.contains_epsilon() && other.contains_epsilon() {
            return true;
        }
        for (set, other_set) in self.array.borrow().iter().zip(other.array.borrow().iter()) {
            if set.intersects(other_set) {
                return true;
            }
        }
        false
    }

    #[inline]
    fn get(&self, ty: T) -> &LitSet {
        &self.array.borrow()[ty.id()]
    }

    #[inline]
    fn get_mut(&mut self, ty: T) -> &mut LitSet {
        &mut self.array.borrow_mut()[ty.id()]
    }
}

pub fn union_impl<T: TokenType>(s: &mut T::Set, o: &T::Set) -> bool {
    let mut changed = false;
    for (set, other_set) in s.borrow_mut().iter_mut().zip(o.borrow().iter()) {
        let next_changed = set.union(other_set);
        changed |= next_changed;
    }
    changed
}

#[cfg(test)]
mod first_set_tests {
    use super::*;

    use crate::test::TestTokenType as T;

    #[test]
    fn insert_epsilon() {
        let mut set = FirstSet::<T>::new();
        assert!(!set.contains_epsilon());
        assert!(!set.contains(None));
        assert_eq!(set.insert_epsilon(), true);
        assert!(set.contains_epsilon());
        assert!(set.contains(None));
        assert_eq!(set.insert_epsilon(), false);
        assert!(set.contains_epsilon());
        assert!(set.contains(None));
    }

    #[test]
    fn insert() {
        let mut set = FirstSet::new();
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

}
