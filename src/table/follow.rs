
use std::any::TypeId;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{BTreeMap, BTreeSet};

use derivative::Derivative;

use crate::token::TokenSrc;
use crate::{SyntaxTree, TokenType};

use super::first::{First, FirstSet};
use super::LitSet;

pub struct FollowBuilder<T: TokenType> {
    pub first: First<T>,
    seen: BTreeSet<TypeId>,
    exprs: Vec<FollowExpr>,
}

impl<T: TokenType> FollowBuilder<T> {
    #[inline(always)]
    pub fn build_recursive<ST: SyntaxTree<T=T>>(&mut self) {
        let t = ST::type_id();
        if self.seen.insert(t) {
            ST::build_follow(self);
        }
    }

    #[inline]
    pub fn add(&mut self, expr: FollowExpr) {
        self.exprs.push(expr);
    }

    /// Build follow table for X -> Y1 | Y2 | ... | Yn
    #[inline]
    pub fn build_for_enum(&mut self, x: TypeId, variants: &[TypeId]) {
        for y in variants {
            // for X -> Yi
            // FOLLOW(Yi) = FOLLOW(Yi) U FOLLOW(X)
            self.add(FollowExpr::union_follow(*y, x));
        }
    }

    /// Build follow table for X -> Y1 Y2 ... Yn
    #[inline]
    pub fn build_for_sequence(&mut self, x: TypeId, sequence: &[TypeId]) {
        let mut set = BTreeSet::new();
        for yi in sequence.iter().rev() {
            // if Yi ... Yn all has epsilon in FIRST(Yi), then FOLLOW(Yi) = FOLLOW(Yi) U FOLLOW(X)
            self.add(FollowExpr::if_epsilon_in_all_first(set.iter(), FollowExpr::union_follow(*yi, x)));
            set.insert(*yi);
        }
        for yi in sequence.windows(2).rev() {
            // for X -> Y1 Y2 ... Yi Yi+1 ... Yn
            // FOLLOW(Yi) = FOLLOW(Yi) U (FIRST(Yi+1) - { epsilon })
            self.add(FollowExpr::union_first_minus_epsilon(yi[0], yi[1]));
        }
    }

    pub fn build(mut self, root: TypeId) -> (First<T>, Follow<T>) {
        let mut map = {
            let mut map = BTreeMap::new();
            let mut root_set = FollowSet::new();
            root_set.insert_eof();
            map.insert(root, root_set);
            map
        };

        let mut changed = true;
        while changed {
            changed = false;
            for expr in &mut self.exprs {
                match expr {
                    FollowExpr::UnionFirstMinusEpsilon(a, b) => {
                        let follow_a = map.entry(*a).or_default();
                        let first_b = self.first.get(b);
                        changed |= follow_a.union_first_minus_epsilon(first_b);
                    }
                    FollowExpr::UnionFollow(a, b) => {
                        let mut follow_a = map.remove(a).unwrap_or_default();
                        if let Some(follow_b) = map.get(b) {
                            changed |= follow_a.union_follow(follow_b);
                        }
                        map.insert(*a, follow_a);
                    }
                    FollowExpr::IfEpsilonInAllFirst(set, inner) => {
                        // keep the sets that don't contain epsilon
                        set.retain(|t| {
                             !self.first.get(t).contains_epsilon()
                        });
                        if set.is_empty() {
                            let mut noop = FollowExpr::Noop;
                            std::mem::swap(inner.as_mut(), &mut noop);
                            *expr = noop;
                            changed = true;
                        }
                    }
                    FollowExpr::Noop => {}
                }
            }
        }

        let follow = Follow {
            map,
            empty: FollowSet::new(),
        };

        (self.first, follow)
    }
}

pub enum FollowExpr {
    Noop,
    /// `FOLLOW(A) = FOLLOW(A) U (FIRST(B) - { epsilon })`
    UnionFirstMinusEpsilon(TypeId, TypeId),
    /// `FOLLOW(A) = FOLLOW(A) U FOLLOW(B)`
    UnionFollow(TypeId, TypeId),
    /// If epsilon is in INTERSECTION(FIRST(A)), then execute the inner expression
    IfEpsilonInAllFirst(BTreeSet<TypeId>, Box<FollowExpr>),
}

impl FollowExpr {
    #[inline]
    pub fn union_first_minus_epsilon(a: TypeId, b: TypeId) -> Self {
        Self::UnionFirstMinusEpsilon(a, b)
    }

    #[inline]
    pub fn union_follow(a: TypeId, b: TypeId) -> Self {
        Self::UnionFollow(a, b)
    }

    #[inline]
    pub fn if_epsilon_in_all_first<'s, Iter: IntoIterator<Item=&'s TypeId>>(set: Iter, expr: FollowExpr) -> Self {
        let set = set.into_iter().copied().collect::<BTreeSet<_>>();
        if set.is_empty() {
            return expr;
        }
        Self::IfEpsilonInAllFirst(set, Box::new(expr))
    }
}

pub struct Follow<T: TokenType> {
    map: BTreeMap<TypeId, FollowSet<T>>,
    empty: FollowSet<T>,
}

#[derive(Derivative)]
#[derivative(Default(new = "true", bound = ""))]
pub struct FollowSet<T: TokenType>(FirstSet<T>);

impl<T: TokenType> FollowSet<T> {
    #[inline]
    pub fn insert_eof(&mut self) {
        self.0.insert_epsilon();
    }

    #[inline]
    pub fn contains_eof(&self) -> bool {
        self.0.contains_epsilon()
    }

    /// Union with a FIRST set. (Epsilon will not be included)
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn union_first_minus_epsilon(&mut self, other: &FirstSet<T>) -> bool {
        self.0.union_minus_epsilon(other)
    }


    /// Union with another FOLLOW set
    ///
    /// Returns if the set is changed
    #[inline]
    pub fn union_follow(&mut self, other: &Self) -> bool {
        let mut changed = false;
        if !self.contains_eof()  && other.contains_eof() {
            self.insert_eof();
            changed = true;
        }
        self.0.union_minus_epsilon(&other.0) || changed
    }
    // #[inline]
    // pub fn contains<'s>(&self, token: Option<TokenSrc<'s, T>>) -> bool {
    //     self.0.contains(token)
    // }
}
