use std::any::TypeId;
use std::collections::BTreeSet;
use itertools::Itertools;
use crate::parser::ParseTree;
use crate::syntax::{
    FirstSet, First, FirstBuilder, Follow, FollowBuilder, Jump, Metadata,
};
use crate::{AbstractSyntaxTree, GrammarError, Lexicon, Parser};
use teleparse_macros::derive_tuple_ast;
const _: () = {
    #[automatically_derived]
    impl<
        A: teleparse::AbstractSyntaxTree,
        B: teleparse::AbstractSyntaxTree<L = <A as teleparse::AbstractSyntaxTree>::L>,
    > teleparse::AbstractSyntaxTree for (A, B) {
        type L = <A as teleparse::AbstractSyntaxTree>::L;
        fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            <A>::build_first(builder);
            <B>::build_first(builder);
            builder.build_sequence(t, &[<A>::type_id(), <B>::type_id()]);
        }
        fn check_left_recursive(
            stack: &mut ::std::vec::Vec<::std::string::String>,
            set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            let t = Self::type_id();
            if !set.insert(t) {
                return Err(
                    teleparse::GrammarError::left_recursion(&stack, &Self::debug()),
                );
            }
            stack.push(Self::debug().into_owned());
            if let Err(e) = <A>::check_left_recursive(stack, set, first) {
                stack.pop();
                set.remove(&t);
                return Err(e);
            }
            let mut temp_stack = ::std::vec::Vec::new();
            let mut temp_set = ::std::collections::BTreeSet::new();
            let (cur_stack, cur_set, need_pop) = if first
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
            if need_pop {
                cur_stack.pop();
                cur_set.remove(&t);
            }
            check
        }
        fn check_first_conflict(
            seen: &mut ::std::collections::BTreeSet<TypeId>,
            first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            let cur = <A>::type_id();
            let cur_first = first.get(&cur);
            let cur_name = <A>::debug();
            #[allow(unused_mut)]
            let mut cur_check = if cur_first.contains_epsilon() {
                cur_first.clone()
            } else {
                teleparse::syntax::FirstSet::new()
            };
            let next = <B>::type_id();
            let next_first = first.get(&next);
            if cur_check.intersects_minus_epsilon(next_first) {
                let cur_name = cur_name.into_owned();
                let next_name = <B>::debug().into_owned();
                let self_name = Self::debug().into_owned();
                let terminals = cur_check
                    .intersection_repr_minus_epsilon(next_first)
                    .into_iter()
                    .collect::<::std::vec::Vec<_>>()
                    .join(", ");
                return Err(
                    teleparse::GrammarError::FirstFollowSeqConflict(
                        self_name,
                        cur_name,
                        next_name,
                        terminals,
                    ),
                );
            }
            <A>::check_first_conflict(seen, first)?;
            <B>::check_first_conflict(seen, first)
        }
        fn build_follow(builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            <A>::build_follow(builder);
            <B>::build_follow(builder);
            builder.build_sequence(t, &[<A>::type_id(), <B>::type_id()]);
        }
        fn check_first_follow_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &teleparse::syntax::First<Self::L>,
            follow: &teleparse::syntax::Follow<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            Self::check_self_first_follow_conflict(first, follow)?;
            <A>::check_first_follow_conflict(seen, first, follow)?;
            <B>::check_first_follow_conflict(seen, first, follow)?;
            Ok(())
        }
        fn build_jump(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &teleparse::syntax::First<Self::L>,
            jump: &mut teleparse::syntax::Jump<Self::L>,
        ) {
            let t = Self::type_id();
            if !seen.insert(t) {
                return;
            }
            <A>::build_jump(seen, first, jump);
            <B>::build_jump(seen, first, jump);
        }
        fn parse_ast<'s>(
            parser: &mut teleparse::parser::Parser<'s, Self::L>,
            meta: &teleparse::syntax::Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let token = parser.peek_token_src();
            let t = Self::type_id();
            let first = meta.first.get(&t);
            if !first.contains(token) {
                return teleparse::syntax::Result::Panic(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([parser.expecting(first.clone())]),
                    ),
                );
            }
            let mut errors = ::std::vec::Vec::new();
            let result = (
                match <A>::parse_ast(parser, meta) {
                    teleparse::syntax::Result::Success(x) => x,
                    teleparse::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    teleparse::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return teleparse::syntax::Result::Panic(errors);
                    }
                },
                match <B>::parse_ast(parser, meta) {
                    teleparse::syntax::Result::Success(x) => x,
                    teleparse::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    teleparse::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return teleparse::syntax::Result::Panic(errors);
                    }
                },
            );
            if errors.is_empty() {
                teleparse::syntax::Result::Success(result)
            } else {
                teleparse::syntax::Result::Recovered(result, errors)
            }
        }
    }
};
const _: () = {
    #[automatically_derived]
    impl<
        A: teleparse::AbstractSyntaxTree,
        B: teleparse::AbstractSyntaxTree<L = <A as teleparse::AbstractSyntaxTree>::L>,
        C: teleparse::AbstractSyntaxTree<L = <A as teleparse::AbstractSyntaxTree>::L>,
    > teleparse::AbstractSyntaxTree for (A, B, C) {
        type L = <A as teleparse::AbstractSyntaxTree>::L;
        fn build_first(builder: &mut teleparse::syntax::FirstBuilder<Self::L>) {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            <A>::build_first(builder);
            <B>::build_first(builder);
            <C>::build_first(builder);
            builder.build_sequence(t, &[<A>::type_id(), <B>::type_id(), <C>::type_id()]);
        }
        fn check_left_recursive(
            stack: &mut ::std::vec::Vec<::std::string::String>,
            set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            let t = Self::type_id();
            if !set.insert(t) {
                return Err(
                    teleparse::GrammarError::left_recursion(&stack, &Self::debug()),
                );
            }
            stack.push(Self::debug().into_owned());
            if let Err(e) = <A>::check_left_recursive(stack, set, first) {
                stack.pop();
                set.remove(&t);
                return Err(e);
            }
            let mut temp_stack = ::std::vec::Vec::new();
            let mut temp_set = ::std::collections::BTreeSet::new();
            let (cur_stack, cur_set, need_pop) = if first
                .get(&<A>::type_id())
                .contains_epsilon()
            {
                (stack, set, true)
            } else {
                stack.pop();
                set.remove(&t);
                (&mut temp_stack, &mut temp_set, false)
            };
            if let Err(e) = <B>::check_left_recursive(cur_stack, cur_set, first) {
                if need_pop {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                return Err(e);
            }
            let (cur_stack, cur_set, need_pop) = if first
                .get(&<B>::type_id())
                .contains_epsilon()
            {
                (cur_stack, cur_set, need_pop)
            } else {
                if need_pop {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                temp_stack.clear();
                temp_set.clear();
                (&mut temp_stack, &mut temp_set, false)
            };
            let check = <C>::check_left_recursive(cur_stack, cur_set, first);
            if need_pop {
                cur_stack.pop();
                cur_set.remove(&t);
            }
            check
        }
        fn check_first_conflict(
            seen: &mut ::std::collections::BTreeSet<TypeId>,
            first: &teleparse::syntax::First<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            let cur = <A>::type_id();
            let cur_first = first.get(&cur);
            let cur_name = <A>::debug();
            #[allow(unused_mut)]
            let mut cur_check = if cur_first.contains_epsilon() {
                cur_first.clone()
            } else {
                teleparse::syntax::FirstSet::new()
            };
            let next = <B>::type_id();
            let next_first = first.get(&next);
            let next_name = <B>::debug();
            if cur_check.intersects_minus_epsilon(next_first) {
                let cur_name = cur_name.into_owned();
                let next_name = next_name.into_owned();
                let self_name = Self::debug().into_owned();
                let terminals = cur_check
                    .intersection_repr_minus_epsilon(next_first)
                    .into_iter()
                    .collect::<::std::vec::Vec<_>>()
                    .join(", ");
                return Err(
                    teleparse::GrammarError::FirstFollowSeqConflict(
                        self_name,
                        cur_name,
                        next_name,
                        terminals,
                    ),
                );
            }
            let (cur, cur_name) = if next_first.contains_epsilon() {
                cur_check.union_minus_epsilon(next_first);
                (cur, cur_name)
            } else {
                cur_check.clear();
                (next, next_name)
            };
            let next = <C>::type_id();
            let next_first = first.get(&next);
            if cur_check.intersects_minus_epsilon(next_first) {
                let cur_name = cur_name.into_owned();
                let next_name = <C>::debug().into_owned();
                let self_name = Self::debug().into_owned();
                let terminals = cur_check
                    .intersection_repr_minus_epsilon(next_first)
                    .into_iter()
                    .collect::<::std::vec::Vec<_>>()
                    .join(", ");
                return Err(
                    teleparse::GrammarError::FirstFollowSeqConflict(
                        self_name,
                        cur_name,
                        next_name,
                        terminals,
                    ),
                );
            }
            <A>::check_first_conflict(seen, first)?;
            <B>::check_first_conflict(seen, first)?;
            <C>::check_first_conflict(seen, first)
        }
        fn build_follow(builder: &mut teleparse::syntax::FollowBuilder<Self::L>) {
            let t = Self::type_id();
            if !builder.visit(t) {
                return;
            }
            <A>::build_follow(builder);
            <B>::build_follow(builder);
            <C>::build_follow(builder);
            builder.build_sequence(t, &[<A>::type_id(), <B>::type_id(), <C>::type_id()]);
        }
        fn check_first_follow_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &teleparse::syntax::First<Self::L>,
            follow: &teleparse::syntax::Follow<Self::L>,
        ) -> ::core::result::Result<(), teleparse::GrammarError> {
            let t = Self::type_id();
            if !seen.insert(t) {
                return Ok(());
            }
            Self::check_self_first_follow_conflict(first, follow)?;
            <A>::check_first_follow_conflict(seen, first, follow)?;
            <B>::check_first_follow_conflict(seen, first, follow)?;
            <C>::check_first_follow_conflict(seen, first, follow)?;
            Ok(())
        }
        fn build_jump(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &teleparse::syntax::First<Self::L>,
            jump: &mut teleparse::syntax::Jump<Self::L>,
        ) {
            let t = Self::type_id();
            if !seen.insert(t) {
                return;
            }
            <A>::build_jump(seen, first, jump);
            <B>::build_jump(seen, first, jump);
            <C>::build_jump(seen, first, jump);
        }
        fn parse_ast<'s>(
            parser: &mut teleparse::parser::Parser<'s, Self::L>,
            meta: &teleparse::syntax::Metadata<Self::L>,
        ) -> teleparse::syntax::Result<Self, Self::L> {
            let token = parser.peek_token_src();
            let t = Self::type_id();
            let first = meta.first.get(&t);
            if !first.contains(token) {
                return teleparse::syntax::Result::Panic(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([parser.expecting(first.clone())]),
                    ),
                );
            }
            let mut errors = ::std::vec::Vec::new();
            let result = (
                match <A>::parse_ast(parser, meta) {
                    teleparse::syntax::Result::Success(x) => x,
                    teleparse::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    teleparse::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return teleparse::syntax::Result::Panic(errors);
                    }
                },
                match <B>::parse_ast(parser, meta) {
                    teleparse::syntax::Result::Success(x) => x,
                    teleparse::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    teleparse::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return teleparse::syntax::Result::Panic(errors);
                    }
                },
                match <C>::parse_ast(parser, meta) {
                    teleparse::syntax::Result::Success(x) => x,
                    teleparse::syntax::Result::Recovered(x, e) => {
                        errors.extend(e);
                        x
                    }
                    teleparse::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return teleparse::syntax::Result::Panic(errors);
                    }
                },
            );
            if errors.is_empty() {
                teleparse::syntax::Result::Success(result)
            } else {
                teleparse::syntax::Result::Recovered(result, errors)
            }
        }
    }
};
