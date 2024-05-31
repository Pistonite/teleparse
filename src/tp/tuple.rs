use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::{prelude::*, ToSpan};
use crate::parser::ParserState;
use crate::table::{LitTable, SyntaxTreeTable, TermSet};
use crate::{Parser, SyntaxTree, SyntaxResult};



impl<A: SyntaxTree + 'static, B: SyntaxTree<T=A::T> + 'static> SyntaxTree for (A, B) {
    type T = A::T;
    type AST = (A::AST, B::AST);

    #[inline]
    fn type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    #[inline]
    fn can_be_empty() -> bool {
        A::can_be_empty() && B::can_be_empty()
    }

    #[inline]
    fn check_left_recursive(stack: &mut Vec<TypeId>, set: &mut BTreeSet<TypeId>) -> bool {
        let t = Self::type_id();
        if set.contains(&t) {
            return true; // is left-recursive
        }
        stack.push(t);
        set.insert(t);
        let check_a = A::check_left_recursive(stack, set);

        let check_b = if A::can_be_empty() {
            B::check_left_recursive(stack, set)
        } else {
            let mut stack = Vec::new();
            let mut set = BTreeSet::new();
            B::check_left_recursive(&mut stack, &mut set)
        };

        stack.pop();
        set.remove(&t);
        return check_a || check_b;
    }

    fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>, 
            lits: &mut LitTable) -> bool {
        s_table.init(Self::type_id(), |s_table| {
            let a_ll1 = A::build_first_table(s_table, lits);
            let b_ll1 = B::build_first_table(s_table, lits);
            let mut first = s_table.get(A::type_id()).into_owned();
            let self_ll1 = if A::can_be_empty() {
                let first_b = s_table.get(B::type_id());
                let collide = first.intersects(&first_b);
                first.union(&first_b);
                !collide
            } else {
                true
            };
            (first, a_ll1 && b_ll1 && self_ll1)
        })
    }

    fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>, 
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>
            ) -> (Cow<'s, TermSet<Self::T>>, bool, bool) {
        let t = Self::type_id();
        let changed = f_table.get_mut(t).union(follows);
        if !changed {
            let mut first = s_table.get(t).into_owned();
            let self_follow = f_table.get(t);
            if Self::can_be_empty() {
                first.remove_empty();
                first.union(&self_follow);
            }
            return (Cow::Owned(first), false, true);
        }

        let (prev_follow, b_ll1) = B::build_follow_table(s_table, f_table, follows);

        let prev_follow = match prev_follow {
            Some(follow) => follow,
            None => return (None, true),
        };

        let (prev_follow, a_ll1) = A::build_follow_table(s_table, f_table, &prev_follow);

        
    }
}

impl<A: SyntaxTree, B: SyntaxTree> ToSpan for (A, B) {
    #[inline]
    fn span(&self) -> Span {
        (self.0.span().lo, self.1.span().hi).into()
    }
}
