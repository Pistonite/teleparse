use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};

use crate::syntax::First;

use crate::Lexicon;

use super::{Follow, FollowSet};

/// Builder for the FOLLOW function
///
/// See [module-level documentation](self) for more information.
#[derive(Debug)]
pub struct FollowBuilder<L: Lexicon> {
    /// The built FIRST table, needed to build the FOLLOW table
    first: First<L>,
    /// Which AST types are already processed
    seen: BTreeSet<TypeId>,
    /// Relations to build the FOLLOW function
    rels: Vec<FollowRel>,
}

impl<L: Lexicon> FollowBuilder<L> {
    /// Create a new builder with the FIRST table
    pub fn new(first: First<L>) -> Self {
        Self {
            first,
            seen: Default::default(),
            rels: Default::default(),
        }
    }

    /// Visit an AST node type and return true if it has not been visited
    /// and rules should be constructed
    #[must_use]
    #[inline]
    pub fn visit(&mut self, ast: TypeId) -> bool {
        self.seen.insert(ast)
    }

    /// Add a [FOLLOW relation](FollowRel) to the builder
    #[inline]
    pub fn add(&mut self, expr: FollowRel) {
        self.rels.push(expr);
    }

    /// Build follow table for X -> Y1 | Y2 | ... | Yn
    #[inline]
    pub fn build_enum(&mut self, x: TypeId, variants: &[TypeId]) {
        for y in variants {
            // for X -> Yi
            // FOLLOW(Yi) = FOLLOW(Yi) U FOLLOW(X)
            self.add(FollowRel::union_follow(*y, x));
        }
    }

    /// Build follow table for X -> Y1 Y2 ... Yn
    pub fn build_sequence(&mut self, x: TypeId, sequence: &[TypeId]) {
        let mut set = BTreeSet::new();
        for yi in sequence.iter().rev() {
            // if Yi ... Yn all has epsilon in FIRST(Yi), then FOLLOW(Yi) = FOLLOW(Yi) U FOLLOW(X)
            self.add(FollowRel::if_epsilon_in_all_first_iter(set.iter(), FollowRel::union_follow(*yi, x)));
            set.insert(*yi);
        }
        for yi in sequence.windows(2).rev() {
            // for X -> Y1 Y2 ... Yi Yi+1 ... Yn
            // FOLLOW(Yi) = FOLLOW(Yi) U (FIRST(Yi+1) - { epsilon })
            self.add(FollowRel::union_first_minus_epsilon(yi[0], yi[1]));
        }
    }

    /// Build the FOLLOW table into a [Follow] instance
    pub fn build(mut self, root: TypeId) -> (First<L>, Follow<L>) {
        // insert EOF for root
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
            // unlike FIRST table, there's no relations that will only execute once
            // so we can skip the retain step here
            for rel in &mut self.rels {
                changed = rel.process_rel(&self.first, &mut map) || changed;
            }
        }

        let follow = Follow {
            map,
            empty: FollowSet::new(),
        };

        (self.first, follow)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FollowRel {
    /// `FOLLOW(A) = FOLLOW(A) U (FIRST(B) - { epsilon })`
    UnionFirstMinusEpsilon(TypeId, TypeId),
    /// `FOLLOW(A) = FOLLOW(A) U FOLLOW(B)`
    UnionFollow(TypeId, TypeId),
    /// If epsilon is in INTERSECTION(FIRST(A)), then execute the inner expression
    ///
    /// Option is used so we can unwrap this in a rusty manner
    IfEpsilonInAllFirst(BTreeSet<TypeId>, Option<Box<FollowRel>>),
}

impl FollowRel {
    #[inline]
    pub fn union_first_minus_epsilon(a: TypeId, b: TypeId) -> Self {
        Self::UnionFirstMinusEpsilon(a, b)
    }

    #[inline]
    pub fn union_follow(a: TypeId, b: TypeId) -> Self {
        Self::UnionFollow(a, b)
    }

    #[inline]
    pub fn if_epsilon_in_all_first_iter<'s, Iter: IntoIterator<Item=&'s TypeId>>(set: Iter, expr: FollowRel) -> Self {
        let set = set.into_iter().copied().collect::<BTreeSet<_>>();
        Self::if_epsilon_in_all_first(set, expr)
    }

    #[inline]
    pub fn if_epsilon_in_all_first(set: BTreeSet<TypeId>, expr: FollowRel) -> Self {
        if set.is_empty() {
            return expr;
        }
        Self::IfEpsilonInAllFirst(set, Some(Box::new(expr)))
    }

    /// Process the relation against the built FIRST table and partially-built FOLLOW table
    ///
    /// Return if anything is changed
    #[must_use]
    fn process_rel<L: Lexicon>(
        &mut self, 
        first: &First<L>,
        follow: &mut BTreeMap<TypeId, FollowSet<L>>) -> bool {
        match self {
            Self::UnionFirstMinusEpsilon(a, b) => {
                let follow_a = follow.entry(*a).or_default();
                let first_b = first.get(b);
                follow_a.union_first(first_b)
            }
            Self::UnionFollow(a, b) => {
                let mut follow_a = follow.remove(a).unwrap_or_default();
                let changed = match follow.get(b) {
                    Some(follow_b) => {
                        follow_a.union_follow(follow_b)
                    }
                    None => false,
                };
                follow.insert(*a, follow_a);
                changed
            }
            Self::IfEpsilonInAllFirst(set, inner) => {
                // keep the sets that don't contain epsilon
                set.retain(|t| {
                    !first.get(t).contains_epsilon()
                });
                // once the set is empty, indicating all of them have epsilon
                // so there intersection has epsilon as well.
                // In which case we can execute the inner relation
                if !set.is_empty() {
                    return false;
                }
                if let Some(mut inner) = std::mem::take(inner) {
                    let changed = inner.process_rel(first, follow);
                    *self = *inner;
                    return changed;
                }
                // should not be reachable since the option is always some
                false
            }
        }
    }
}
