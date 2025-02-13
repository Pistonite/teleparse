use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};

use derivative::Derivative;

use crate::Lexicon;

use super::{First, FirstSet};

/// Builder of the FIRST function.
///
/// See [module-level documentation](super) for more information.
#[derive(Derivative, Debug)]
#[derivative(Default(new = "true", bound = ""))]
pub struct FirstBuilder<L: Lexicon> {
    // /// Which AST types are already processed
    // seen: BTreeMap<TypeId, String>,
    /// Relations to build the FIRST function
    rels: Vec<FirstRel<L>>,
}

impl<L: Lexicon> FirstBuilder<L> {
    // /// Visit an AST node type and return true if it has not been visited
    // /// and rules should be constructed
    // #[must_use]
    // #[inline]
    // pub fn visit(&mut self, ast: TypeId, name: &str) -> bool {
    //     if self.seen.contains_key(&ast) {
    //         return false;
    //     }
    //     self.seen.insert(ast, name.to_string());
    //     true
    // }

    /// Add a [FIRST relation](FirstRel) to the builder
    #[inline]
    pub fn add(&mut self, expr: FirstRel<L>) {
        self.rels.push(expr);
    }

    /// Build for X -> Y1 | Y2 | ... | Yn
    #[inline]
    pub fn build_union(&mut self, x: TypeId, variants: &[TypeId]) {
        for y in variants {
            self.add(FirstRel::union_minus_epsilon(x, *y));
            self.add(FirstRel::if_epsilon_in_all_iter(
                [y],
                FirstRel::insert_epsilon(x),
            ));
        }
    }

    /// Build for X -> Y1 Y2 ... Yn
    #[inline]
    pub fn build_sequence(&mut self, x: TypeId, sequence: &[TypeId]) {
        let mut set = BTreeSet::new();
        for yi in sequence {
            // if Y1 ... Yi-1 can all produce epsilon, add FIRST(Yi) - { epsilon }
            self.add(FirstRel::if_epsilon_in_all_iter(
                set.iter(),
                FirstRel::union_minus_epsilon(x, *yi),
            ));
            set.insert(*yi);
        }
        // if all Y1 ... Yn can produce epsilon, add epsilon
        self.add(FirstRel::if_epsilon_in_all(
            set,
            FirstRel::insert_epsilon(x),
        ));
    }

    /// Build the FIRST table into a [First] instance
    pub fn build(self) -> First<L> {
        let mut first = BTreeMap::<TypeId, FirstSet<L>>::new();
        let mut rels = self.rels;
        let mut changed = true;

        while changed {
            changed = Self::process_rels(&mut first, &mut rels);
        }

        First::new(first)
    }

    /// Process the relations once and return if anything changed
    #[must_use]
    fn process_rels(
        first: &mut BTreeMap<TypeId, FirstSet<L>>,
        rels: &mut Vec<FirstRel<L>>,
    ) -> bool {
        let mut changed = false;
        rels.retain_mut(|rel| {
            let (chg, retain) = rel.process_rel(first);
            changed |= chg;
            retain
        });
        changed
    }
}

#[derive(Debug, PartialEq)]
pub enum FirstRel<L: Lexicon> {
    /// `FIRST(A) = FIRST(A) U Insert`
    Insert(TypeId, FirstInsert<L>),
    /// `FIRST(A) = FIRST(A) U (FIRST(B) - { epsilon })`
    UnionMinusEpsilon(TypeId, TypeId),
    /// If epsilon is in INTERSECTION(FIRST(A)), then execute the inner expression
    ///
    /// Option is used so we can unwrap this in a rusty manner
    IfEpsilonInAll(BTreeSet<TypeId>, Option<Box<FirstRel<L>>>),
}

impl<L: Lexicon> FirstRel<L> {
    #[inline]
    pub fn insert_epsilon(t: TypeId) -> Self {
        Self::Insert(t, FirstInsert::Epsilon)
    }

    #[inline]
    pub fn insert_token(t: TypeId, token: L, lit: Option<&'static str>) -> Self {
        Self::Insert(t, FirstInsert::Token(token, lit))
    }

    #[inline]
    pub fn union_minus_epsilon(a: TypeId, b: TypeId) -> Self {
        Self::UnionMinusEpsilon(a, b)
    }

    #[inline]
    pub fn if_epsilon_in_all_iter<'s, Iter: IntoIterator<Item = &'s TypeId>>(
        set: Iter,
        expr: FirstRel<L>,
    ) -> Self {
        let set = set.into_iter().copied().collect::<BTreeSet<_>>();
        Self::if_epsilon_in_all(set, expr)
    }

    #[inline]
    pub fn if_epsilon_in_all(set: BTreeSet<TypeId>, expr: FirstRel<L>) -> Self {
        if set.is_empty() {
            return expr;
        }
        Self::IfEpsilonInAll(set, Some(Box::new(expr)))
    }

    /// Process the relation against the partially-built FIRST table
    ///
    /// Return (changed, should_retain)
    #[must_use]
    fn process_rel(&mut self, first: &mut BTreeMap<TypeId, FirstSet<L>>) -> (bool, bool) {
        match self {
            Self::Insert(t, insert) => {
                let set = first.entry(*t).or_default();
                let changed = match insert {
                    FirstInsert::Epsilon => set.insert_epsilon(),
                    FirstInsert::Token(token, lit) => set.insert(*token, *lit),
                };
                // after something is added to the set, it's always going
                // to be there, so we can skip further insertions
                (changed, false)
            }
            Self::UnionMinusEpsilon(a, b) => {
                let mut first_a = first.remove(a).unwrap_or_default();
                let changed = match first.get(b) {
                    Some(first_b) => first_a.union_minus_epsilon(first_b),
                    None => false,
                };
                first.insert(*a, first_a);
                // for union relations, we need to keep unioning until nothing is added
                (changed, true)
            }
            Self::IfEpsilonInAll(set, inner) => {
                // keep the sets that don't contain epsilon
                set.retain(|t| match first.get(t) {
                    Some(set) => !set.contains_epsilon(),
                    None => true,
                });
                // once the set is empty, indicating all of them have epsilon
                // so there intersection has epsilon as well.
                // In which case we can execute the inner relation
                if !set.is_empty() {
                    // otherwise, nothing changed, and we need to keep this relation
                    return (false, true);
                }
                if let Some(mut inner) = std::mem::take(inner) {
                    let (changed, retain) = inner.process_rel(first);
                    *self = *inner;
                    return (changed, retain);
                }

                // should not be reachable since the option is always some
                (false, false)
            }
        }
    }
}

/// Insert operation for FIRST set. Does not depend on other FIRST sets.
#[derive(Debug, PartialEq)]
pub enum FirstInsert<L: Lexicon> {
    /// Union with `{ (T, lit) }`
    Token(L, Option<&'static str>),
    /// Union with `{ epsilon }`
    Epsilon,
}
