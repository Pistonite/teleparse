use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;
use itertools::Itertools;
use crate::syntax::{
    FirstSet, First, FirstBuilder, Follow, FollowBuilder, Jump, Metadata,
};
use crate::{AbstractSyntaxTree, GrammarError, Parser};
impl<A: AbstractSyntaxTree, B: AbstractSyntaxTree<L = A::L>> AbstractSyntaxTree
for (A, B) {
    type L = A::L;
    #[inline]
    fn build_first(builder: &mut FirstBuilder<Self::L>) {
        {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            <A>::build_first(builder);
            <B>::build_first(builder);
            builder.build_sequence(t, &[<A>::type_id(), <B>::type_id()]);
        }
    }
    #[inline]
    fn check_left_recursive(
        stack: &mut Vec<String>,
        set: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
    ) -> Result<(), GrammarError> {
        {
            let t = Self::type_id();
            if !set.insert(t) {
                return Err(GrammarError::left_recursion(&stack, &Self::debug()));
            }
            stack.push(Self::debug().into_owned());
            if let Err(e) = <A>::check_left_recursive(stack, set, first) {
                stack.pop();
                set.remove(&t);
                return Err(e);
            }
            let mut temp_stack = Vec::new();
            let mut temp_set = BTreeSet::new();
            let (cur_stack, cur_set, cur_is_main) = if first
                .get(&<A>::type_id())
                .contains_epsilon()
            {
                (stack, set, true)
            } else {
                stack.pop();
                set.remove(&t);
                (&mut temp_stack, &mut temp_set, false)
            };
            let check = <B>::check_left_recursive(cur_stack, cur_set, first);
            if cur_is_main {
                cur_stack.pop();
                cur_set.remove(&t);
            }
            check
        }
    }
    #[inline]
    fn check_first_conflict(
        seen: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
    ) -> Result<(), GrammarError> {
        {
            <A>::check_first_conflict(seen, first)?;
            <B>::check_first_conflict(seen, first)?;
            let current = <A>::type_id();
            let first_set = first.get(&current);
            let current_symbol = <A>::debug();
            #[allow(unused_mut)]
            let mut current_check = if first_set.contains_epsilon() {
                first_set.clone()
            } else {
                FirstSet::new()
            };
            let next = <B>::type_id();
            let next_first = first.get(&next);
            if !next_first.contains_epsilon() {
                if current_check.intersects_minus_epsilon(next_first) {
                    let current_symbol = current_symbol.into_owned();
                    let next_symbol = <B>::debug();
                    let next_symbol = next_symbol.into_owned();
                    let self_symbol = Self::debug().into_owned();
                    let terminals = current_check
                        .intersection_repr_minus_epsilon(next_first)
                        .into_iter()
                        .join(", ");
                    return Err(
                        GrammarError::FirstFollowStringConflict(
                            self_symbol,
                            current_symbol,
                            next_symbol,
                            terminals,
                        ),
                    );
                }
            }
            Ok(())
        }
    }
    #[inline]
    fn build_follow(builder: &mut FollowBuilder<Self::L>) {
        {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            <A>::build_follow(builder);
            <B>::build_follow(builder);
            builder.build_sequence(t, &[<A>::type_id(), <B>::type_id()]);
        }
    }
    #[inline]
    fn check_first_follow_conflict(
        seen: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
        follow: &Follow<Self::L>,
    ) -> Result<(), GrammarError> {
        {
            <A>::check_first_follow_conflict(seen, first, follow)?;
            <B>::check_first_follow_conflict(seen, first, follow)
        }
    }
    #[inline]
    fn build_jump(
        seen: &mut BTreeSet<TypeId>,
        first: &First<Self::L>,
        jump: &mut Jump<Self::L>,
    ) {
        {
            if seen.insert(Self::type_id()) {
                <A>::build_jump(seen, first, jump);
                <B>::build_jump(seen, first, jump);
            }
        }
    }
    #[inline]
    fn parse<'s>(
        parser: &mut Parser<'s, Self::L>,
        meta: &Metadata<Self::L>,
    ) -> crate::syntax::Result<Self, Self::L> {
        {
            let token = parser.peek_token_src();
            let t = Self::type_id();
            let first = meta.first.get(&t);
            if !first.contains(token) {
                return crate::syntax::Result::Panic(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([parser.expecting(first.clone())]),
                    ),
                );
            }
            let mut errors = Vec::new();
            let result = (
                match <A>::parse(parser, meta) {
                    crate::syntax::Result::Success(x) => x,
                    crate::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    crate::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return crate::syntax::Result::Panic(errors);
                    }
                },
                match <B>::parse(parser, meta) {
                    crate::syntax::Result::Success(x) => x,
                    crate::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    crate::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return crate::syntax::Result::Panic(errors);
                    }
                },
            );
            if errors.is_empty() {
                crate::syntax::Result::Success(result)
            } else {
                crate::syntax::Result::Recovered(result, errors)
            }
        }
    }
}
