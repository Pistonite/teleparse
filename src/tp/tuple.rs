use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::table::first::{First, FirstBuilder};
use crate::table::follow::FollowBuilder;
use crate::{prelude::*, ToSpan};
use crate::parser::ParserState;
use crate::{SyntaxTree, SyntaxResult};



impl<A: SyntaxTree, B: SyntaxTree<T=A::T>> SyntaxTree for (A, B) {
    type T = A::T;
    type AST = (A::AST, B::AST);

    #[inline]
    fn produces_epsilon() -> bool {
        A::produces_epsilon() && B::produces_epsilon()
    }

    #[inline]
    fn check_left_recursive(stack: &mut Vec<TypeId>, set: &mut BTreeSet<TypeId>) -> Option<Vec<TypeId>> {
        let t = Self::type_id();
        if set.contains(&t) {
            let mut stack = stack.clone();
            stack.push(t);
            return Some(stack);
        }
        stack.push(t);
        set.insert(t);
        let check_a = A::check_left_recursive(stack, set);

        let check_b = if A::produces_epsilon() {
            B::check_left_recursive(stack, set)
        } else {
            let mut stack = Vec::new();
            let mut set = BTreeSet::new();
            B::check_left_recursive(&mut stack, &mut set)
        };

        stack.pop();
        set.remove(&t);
        if check_a.is_some() {
            return check_a;
        }
        if check_b.is_some() {
            return check_b;
        }
        return None;
    }

    #[inline]
    fn build_first(builder: &mut FirstBuilder<Self::T>) {
        builder.build_recursive::<A>();
        builder.build_recursive::<B>();
        builder.build_for_sequence(Self::type_id(), &[A::type_id(), B::type_id()]);
    }

    #[inline]
    fn check_conflict(first: &First<Self::T>) -> bool {
        first.has_collision(&A::type_id(), &B::type_id())
    }

    #[inline]
    fn build_follow(builder: &mut FollowBuilder<Self::T>) {
        builder.build_recursive::<A>();
        builder.build_recursive::<B>();
        builder.build_for_sequence(Self::type_id(), &[A::type_id(), B::type_id()]);
    }
}
