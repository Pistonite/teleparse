
use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};

use derivative::Derivative;
use itertools::Itertools;

use crate::syntax::FirstSet;
use crate::{GrammarError, Lexicon, Production};

use super::{DebugFirst, DebugFollow, DebugJump, First, FirstBuilder, FirstRel, Follow, FollowBuilder, Jump};

#[derive(Derivative)]
#[derivative(Default(new="true", bound=""))]
pub struct MetadataBuilder<L: Lexicon> {
    pub names: BTreeMap<TypeId, String>,
    pub productions: BTreeMap<TypeId, ProductionEntry>,
    pub first_builder: FirstBuilder<L>,
}

impl<L: Lexicon> MetadataBuilder<L> {

    #[inline]
    #[must_use]
    pub fn visit<F: FnOnce() -> String>(&mut self, t: TypeId, name: F) -> bool {
        if self.names.contains_key(&t) {
            return false;
        }
        self.names.insert(t, name());
        true
    }

    pub fn add_epsilon(&mut self, t: TypeId) {
        self.productions.insert(t, ProductionEntry::Terminal);
        self.first_builder.add(FirstRel::insert_epsilon(t))
    }

    pub fn add_terminal(&mut self, t: TypeId, ty: L, lit: Option<&'static str>) {
        self.productions.insert(t, ProductionEntry::Terminal);
        self.first_builder.add(FirstRel::insert_token(t, ty, lit))
    }

    pub fn add_sequence(&mut self, t: TypeId, types: &[TypeId]) {
        self.productions.insert(t, ProductionEntry::Sequence(types.to_vec()));
        self.first_builder.build_sequence(t, types);
    }

    pub fn add_union(&mut self, t: TypeId, types: &[TypeId]) {
        self.productions.insert(t, ProductionEntry::Union(types.to_vec()));
        self.first_builder.build_union(t, types);
    }

    pub fn build(self, root: TypeId) -> Result<Metadata<L>, GrammarError> {
        let names = self.names;
        let prods = self.productions;
        let first = self.first_builder.build();
        let mut seen = BTreeSet::new();
        let mut set = BTreeSet::new();
        let mut stack = Vec::new();
        check_left_recursive(
            &names,
            &prods,
            root,
            &mut seen,
            &mut set,
            &mut stack,
            &first
        )?;
        let mut follow_builder = FollowBuilder::new(&first);
        for (t, prod) in &prods {
            match prod {
                ProductionEntry::Terminal => {},
                ProductionEntry::Sequence(types) => {
                    follow_builder.build_sequence(*t, types);
                },
                ProductionEntry::Union(types) => {
                    follow_builder.build_union(*t, types);
                }
            }
        }
        let follow = follow_builder.build(root);
        seen.clear();
        check_conflicts(
            &names,
            &prods,
            root,
            &mut seen,
            &first,
            &follow
        )?;
        let mut jump = Jump::new();
        seen.clear();
        build_jump(
            &names,
            &prods,
            root,
            &mut seen,
            &first,
            &mut jump,
        );
        Ok(Metadata {
            names,
            first,
            follow,
            jump,
        })
    }


}
fn check_left_recursive<L: Lexicon>(
    names: &BTreeMap<TypeId, String>, 
    prods: &BTreeMap<TypeId, ProductionEntry>, 
    t: TypeId,
    seen: &mut BTreeSet<TypeId>,
    set: &mut BTreeSet<TypeId>,
    stack: &mut Vec<TypeId>,
    first: &First<L>
) -> Result<(), GrammarError> {
    if !seen.insert(t) {
        return Ok(());
    }
    if !set.insert(t) {
        let stack = stack.iter().map(|t| names.get(t).map(|x|x.as_str()).unwrap_or_default()).collect::<Vec<_>>();
        let current = names.get(&t).map(|x|x.as_str()).unwrap_or_default();
        return Err(GrammarError::left_recursion(&stack, current));
    }
    match prods.get(&t) {
        None => {
            set.remove(&t);
        },
        Some(ProductionEntry::Terminal) => {
            set.remove(&t);
        },
        Some(ProductionEntry::Union(types)) => {
            stack.push(t);
            for inner in types.iter() {
                let result = check_left_recursive(
                    names,
                    prods,
                    *inner,
                    seen,
                    set,
                    stack,
                    first
                );
                if let Err(e) = result {
                    stack.pop();
                    set.remove(&t);
                    return Err(e);
                }
            }
            stack.pop();
            set.remove(&t);
        },
        Some(ProductionEntry::Sequence(types)) => {
            stack.push(t);
            let mut iter = types.iter();
            let inner_first = match iter.next() {
                Some(inner) => inner,
                None => {
                    stack.pop();
                    set.remove(&t);
                    return Ok(());
                }
            };
            let result = check_left_recursive(
                names,
                prods,
                *inner_first,
                seen,
                set,
                stack,
                first
            );
            if let Err(e) = result {
                stack.pop();
                set.remove(&t);
                return Err(e);
            }
            let mut temp_stack = Vec::new();
            let mut temp_set = BTreeSet::new();

            let (mut cur_stack, mut cur_set, mut need_pop) = if first.get(inner_first).contains_epsilon() {
                (stack, set, true)
            } else {
                stack.pop();
                set.remove(&t);
                (&mut temp_stack, &mut temp_set, false)
            };

            for inner in iter {
                let result = check_left_recursive(
                    names,
                    prods,
                    *inner,
                    seen,
                    cur_set,
                    cur_stack,
                    first
                );
                if let Err(e) = result {
                    if need_pop {
                        cur_stack.pop();
                        cur_set.remove(&t);
                    }
                    return Err(e);
                }
                if !first.get(inner).contains_epsilon() {
                    if need_pop {
                        cur_stack.pop();
                        cur_set.remove(&t);
                    }
                    temp_stack.clear();
                    temp_set.clear();
                    cur_stack = &mut temp_stack;
                    cur_set = &mut temp_set;
                    need_pop = false;
                };
            }
            if need_pop {
                cur_stack.pop();
                cur_set.remove(&t);
            }
        },
    }

    Ok(())
}

fn check_conflicts<L: Lexicon>(
    names: &BTreeMap<TypeId, String>, 
    prods: &BTreeMap<TypeId, ProductionEntry>, 
    t: TypeId,
    seen: &mut BTreeSet<TypeId>,
    first: &First<L>,
    follow: &Follow<L>
) -> Result<(), GrammarError>{
            if !seen.insert(t) {
                return Ok(());
            }
    match prods.get(&t) {
        None => {},
        Some(ProductionEntry::Terminal) => {},
        Some(ProductionEntry::Union(types)) => {
            let mut check_set = FirstSet::new();
            for inner in types.iter() {
                let first_set = first.get(inner);
                if check_set.intersects(&first_set) {
                    let self_name = names.get(&t).map(|x|x.as_str()).unwrap_or_default();
                    let produce_name = names.get(inner).map(|x|x.as_str()).unwrap_or_default();
                    let intersection = check_set
                        .intersection_repr(&first_set)
                        .into_iter()
                        .join(", ");
                    return Err(GrammarError::FirstFirstConflict(
                        self_name.to_string(),
                        produce_name.to_string(),
                        intersection));
                }
                check_set.union(&first_set);
            }
            check_self_first_follow_conflict(
                names,
                t,
                first,
                follow
            )?;
            for inner in types.iter() {
                check_conflicts(
                    names,
                    prods,
                    *inner,
                    seen,
                    first,
                    follow
                )?;
            }
        },
        Some(ProductionEntry::Sequence(types)) => {
            let mut iter = types.iter();
            let mut cur = match iter.next() {
                Some(inner) => inner,
                None => return Ok(()),
            };
            let cur_first = first.get(cur);
            let mut cur_check = if cur_first.contains_epsilon() {
                cur_first.clone()
            } else {
                FirstSet::new()
            };
            for inner in iter {
                let next_first = first.get(inner);
                if cur_check.intersects_minus_epsilon(next_first) {
                    let cur_name = names.get(cur).cloned().unwrap_or_default();
                    let next_name = names.get(inner).cloned().unwrap_or_default();
                    let self_name = names.get(&t).cloned().unwrap_or_default();
                    let terminals = cur_check
                        .intersection_repr_minus_epsilon(next_first)
                        .into_iter()
                        .join(", ");
                    return Err(GrammarError::FirstFollowSeqConflict(self_name, cur_name, next_name, terminals));
                }
                if next_first.contains_epsilon() {
                    cur_check.union_minus_epsilon(next_first);
                } else {
                    cur_check.clear();
                    cur = inner;
                }
            }
            check_self_first_follow_conflict(
                names,
                t,
                first,
                follow
            )?;
            for inner in types.iter() {
                check_conflicts(
                    names,
                    prods,
                    *inner,
                    seen,
                    first,
                    follow
                )?;
            }
        }
    }

    Ok(())
}

fn check_self_first_follow_conflict<L: Lexicon>(
    names: &BTreeMap<TypeId, String>,
    t: TypeId,
    first: &First<L>, 
    follow: &Follow<L>
) -> Result<(), GrammarError> {
    let first_set = first.get(&t);
    if !first_set.contains_epsilon() {
        return Ok(());
    }
    let follow_set = follow.get(&t);
    if follow_set.intersects_first(first_set) {
        let name = names.get(&t).cloned().unwrap_or_default();
        let intersection = follow_set.intersection_repr_first(first_set).into_iter().join(", ");
        Err(GrammarError::FirstFollowConflict(name, intersection))
    } else {
        Ok(())
    }
}

fn build_jump<L: Lexicon>(
    names: &BTreeMap<TypeId, String>,
    prods: &BTreeMap<TypeId, ProductionEntry>,
    t: TypeId,
    seen: &mut BTreeSet<TypeId>,
    first: &First<L>,
    jump: &mut Jump<L>
) {
    if !seen.insert(t) {
        return;
    }
    match prods.get(&t) {
        None => {},
        Some(ProductionEntry::Terminal) => {},
        Some(ProductionEntry::Sequence(types)) => {
            for inner in types.iter() {
                build_jump(
                    names,
                    prods,
                    *inner,
                    seen,
                    first,
                    jump
                );
            }
        },
        Some(ProductionEntry::Union(types)) => {
            for (i, inner) in types.iter().enumerate() {
                let first_set = first.get(inner);
                jump.register(t, first_set, i);
                build_jump(
                    names,
                    prods,
                    *inner,
                    seen,
                    first,
                    jump
                );
            }
        }
    }
}

pub enum ProductionEntry {
    Terminal,
    Sequence(Vec<TypeId>),
    Union(Vec<TypeId>),
}

pub struct Metadata<L: Lexicon>{
    pub names: BTreeMap<TypeId, String>,
    pub first: First<L>,
    pub follow: Follow<L>,
    pub jump: Jump<L>,
}

impl<L: Lexicon> std::fmt::Debug for Metadata<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metadata")
            .field("first", &DebugFirst(&self.first, &self.names))
            .field("follow", &DebugFollow(&self.follow, &self.names))
            .field("jump", &DebugJump(&self.jump, &self.names))
            .finish()
    }
}

impl<L: Lexicon> Metadata<L> {
    pub fn build_for<T: Production<L=L>>() -> Result<Self, GrammarError> {
        let _lexer = L::lexer("")?;
        let mut builder = MetadataBuilder::new();
        T::register(&mut builder);
        builder.build(T::id())
    }
    
}
