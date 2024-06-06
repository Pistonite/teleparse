use std::any::TypeId;
use std::collections::BTreeSet;

use itertools::Itertools;

use crate::parser::ParseTree;
use crate::syntax::{FirstSet, First, FirstBuilder, Follow, FollowBuilder, Jump, Metadata};
use crate::{AbstractSyntaxTree, GrammarError, Lexicon, Parser};

macro_rules! check_left_recursive_impl {
    ($stack:ident, $set:ident, $first:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        let t = Self::type_id();
        if !$set.insert(t) {
            return Err(GrammarError::left_recursion(&$stack, &Self::debug()));
        }
        $stack.push(Self::debug().into_owned());
        if let Err(e) = <$elem1>::check_left_recursive($stack, $set, $first) {
            $stack.pop();
            $set.remove(&t);
            return Err(e);
        }

        let mut temp_stack = Vec::new();
        let mut temp_set = BTreeSet::new();

        let (cur_stack, cur_set, cur_is_main) = if $first.get(&<$elem1>::type_id()).contains_epsilon() {
            ($stack, $set, true)
        } else {
            $stack.pop();
            $set.remove(&t);
            (&mut temp_stack, &mut temp_set, false)
        };

        $(
            if let Err(e) = <$elem>::check_left_recursive(cur_stack, cur_set, $first) {
                if cur_is_main {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                return Err(e);
            }
            let (cur_stack, cur_set, cur_is_main) = if $first.get(&<$elem>::type_id()).contains_epsilon() {
                (cur_stack, cur_set, cur_is_main)
            } else {
                if cur_is_main {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                temp_stack.clear();
                temp_set.clear();
                (&mut temp_stack, &mut temp_set, false)
            };
        )*

        let check = <$last>::check_left_recursive(cur_stack, cur_set, $first);
        if cur_is_main {
            cur_stack.pop();
            cur_set.remove(&t);
        }
        check
    }}
}

macro_rules! build_first_impl {
    ($builder:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        let t = Self::type_id();
        if !$builder.visit(t) {
            return;
        }
        <$elem1>::build_first($builder);
        $(
            <$elem>::build_first($builder);
        )*
        <$last>::build_first($builder);
        $builder.build_sequence(t, &[<$elem1>::type_id(), $(<$elem>::type_id(),)* <$last>::type_id()]);
    }}
}

macro_rules! build_follow_impl {
    ($builder:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        let t = Self::type_id();
        if !$builder.visit(t) {
            return;
        }
        <$elem1>::build_follow($builder);
        $(
            <$elem>::build_follow($builder);
        )*
        <$last>::build_follow($builder);
        $builder.build_sequence(t, &[<$elem1>::type_id(), $(<$elem>::type_id(),)* <$last>::type_id()]);
    }}
}

macro_rules! check_first_conflict_impl {
    ($seen:ident, $first:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        <$elem1>::check_first_conflict($seen, $first)?;
        $(<$elem>::check_first_conflict($seen, $first)?;)*
        <$last>::check_first_conflict($seen, $first)?;
        let current = <$elem1>::type_id();
        let first_set = $first.get(&current);
        let current_symbol = <$elem1>::debug();
        #[allow(unused_mut)]
        let mut current_check = if first_set.contains_epsilon() {
            first_set.clone()
        } else {
            FirstSet::new()
        };
        $(
            let next = <$elem>::type_id();
            let next_first = $first.get(&next);
            let next_symbol = <$elem>::debug();
            if next_first.contains_epsilon() {
                current_check.union_minus_epsilon(next_first);
            } else {
                if current_check.intersects_minus_epsilon(next_first) {
                    let current_symbol = current_symbol.into_owned();
                    let next_symbol = next_symbol.into_owned();
                    let self_symbol = Self::debug().into_owned();
                    let terminals = current_check
                        .intersection_repr_minus_epsilon(next_first).into_iter().join(", ");
                    return Err(GrammarError::FirstFollowStringConflict(self_symbol, current_symbol, next_symbol, terminals));
                }
                current_check.clear();
            }
            let current = next;
            let current_symbol = next_symbol;
        )*
        let next = <$last>::type_id();
        let next_first = $first.get(&next);
            if !next_first.contains_epsilon() {
                if current_check.intersects_minus_epsilon(next_first) {
                    let current_symbol = current_symbol.into_owned();
                    let next_symbol = <$last>::debug();
                    let next_symbol = next_symbol.into_owned();
                    let self_symbol = Self::debug().into_owned();
                    let terminals = current_check
                        .intersection_repr_minus_epsilon(next_first).into_iter().join(", ");
                    return Err(GrammarError::FirstFollowStringConflict(self_symbol, current_symbol, next_symbol, terminals));
                }
            }

        Ok(())
        
    }}
}

macro_rules! check_first_follow_conflict_impl {
    ($seen:ident, $first:ident, $follow:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        <$elem1>::check_first_follow_conflict($seen, $first, $follow)?;
        $(<$elem>::check_first_follow_conflict($seen, $first, $follow)?;)*
        <$last>::check_first_follow_conflict($seen, $first, $follow)
    }}
}

macro_rules! build_jump_impl {
    ($seen:ident, $first:ident, $jump:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        if $seen.insert(Self::type_id()) {
            <$elem1>::build_jump($seen, $first, $jump);
            $(
                <$elem>::build_jump($seen, $first, $jump);
            )*
            <$last>::build_jump($seen, $first, $jump);
        }
    }}
}

macro_rules! parse_impl {
    ($parser:ident, $meta:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        let token = $parser.peek_token_src();
        let t = Self::type_id();
        let first = $meta.first.get(&t);
        if !first.contains(token) {
            return $crate::syntax::Result::Panic(vec![$parser.expecting(first.clone())]);
        }
        let mut errors = Vec::new();
        let result = (
            match <$elem1>::parse($parser, $meta) {
                $crate::syntax::Result::Success(x) => x,
                $crate::syntax::Result::Recovered(x, e) => { errors.extend(e); x },
                $crate::syntax::Result::Panic(e) => {
                    errors.extend(e);
                    return $crate::syntax::Result::Panic(errors);
                }
            },  $(

                match <$elem>::parse($parser, $meta) {
                    $crate::syntax::Result::Success(x) => x,
                    $crate::syntax::Result::Recovered(x, e) => { errors.extend(e); x },
                    $crate::syntax::Result::Panic(e) => {
                        errors.extend(e);
                        return $crate::syntax::Result::Panic(errors);
                    }
                },

            )*
            match <$last>::parse($parser, $meta) {
                $crate::syntax::Result::Success(x) => x,
                $crate::syntax::Result::Recovered(x, e) => { errors.extend(e); x },
                $crate::syntax::Result::Panic(e) => {
                    errors.extend(e);
                    return $crate::syntax::Result::Panic(errors);
                }
            }
        );

        if errors.is_empty() {
            $crate::syntax::Result::Success(result)
        } else {
            $crate::syntax::Result::Recovered(result, errors)
        }
    }}
}

macro_rules! derive_tuple_ast {
    ($elem1:tt $( + $elem:tt)* | $last:tt) => {

const _: () = {
    #[automatically_derived]
    impl<$elem1: AbstractSyntaxTree,
        $($elem: AbstractSyntaxTree<L=$elem1::L>,)* 
        $last: AbstractSyntaxTree<L=$elem1::L>
    > AbstractSyntaxTree for ($elem1, $($elem,)*  $last) {
        type L = $elem1::L;
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            build_first_impl!(builder, $elem1, $($elem),* | $last);
        }

        #[inline]
        fn check_left_recursive(stack: &mut Vec<String>, set: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError> {
            check_left_recursive_impl!(stack, set, first, $elem1, $($elem),* | $last)
        }

        #[inline]
        fn check_first_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError> {
            check_first_conflict_impl!(seen, first, $elem1, $($elem),* | $last)
        }

        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::L>) {
            build_follow_impl!(builder, $elem1, $($elem),* | $last)
        }

        #[inline]
        fn check_first_follow_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, follow: &Follow<Self::L>) -> Result<(), GrammarError> {
            check_first_follow_conflict_impl!(seen, first, follow, $elem1, $($elem),* | $last)
        }

        #[inline]
        fn build_jump(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, jump: &mut Jump<Self::L>) {
            build_jump_impl!(seen, first, jump, $elem1, $($elem),* | $last)
        }

        #[inline]
        fn parse<'s>( parser: &mut Parser<'s, Self::L>, meta: &Metadata<Self::L>,) -> crate::syntax::Result<Self, Self::L> {
            parse_impl!(parser, meta, $elem1, $($elem),* | $last)
        }
    }
};

    }
}

macro_rules! derive_tuple_parse_tree {
    ($elem1:tt $( + $elem:tt)* | $last:tt) => {
const _: () = {
    #[automatically_derived]
    impl<L: Lexicon, $elem1: ParseTree,
        $($elem: ParseTree,)*
        $last: ParseTree
    > ParseTree for ($elem1, $($elem,)* $last) 
    where
        $elem1::AST: AbstractSyntaxTree<L=L>,
        $($elem::AST: AbstractSyntaxTree<L=L>,)*
        $last::AST: AbstractSyntaxTree<L=L>
    {
        type AST = ($elem1::AST, $($elem::AST,)* $last::AST);

        fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, L>) -> Self {
            #[allow(non_snake_case)]
            let (a, $($elem,)* b) = ast;
            (<$elem1>::from_ast(a, parser), $(<$elem>::from_ast($elem, parser),)* <$last>::from_ast(b, parser))
        }
    }
};

    }
}

derive_tuple_ast!(A | B);
derive_tuple_ast!(A + B | C);
derive_tuple_ast!(A + B + C | D);
derive_tuple_ast!(A + B + C + D | E);
derive_tuple_parse_tree!(A | B);
derive_tuple_parse_tree!(A + B | C);
derive_tuple_parse_tree!(A + B + C | D);
derive_tuple_parse_tree!(A + B + C + D | E);
