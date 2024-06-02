use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use itertools::Itertools;

use crate::root::RootMetadata;
use crate::table::first::{First, FirstBuilder, FirstSet};
use crate::table::follow::{Follow, FollowBuilder};
use crate::table::parsing::Parsing;
use crate::{prelude::*, LL1Error, ToSpan};
use crate::parser::{Parser, ParserState};
use crate::{SyntaxTree, AstResult};

macro_rules! produces_epsilon_impl {
    ($elem1:ty, $($elem:ty),* | $last:ty) => {{
        <$elem1>::produces_epsilon()
        $(&& <$elem>::produces_epsilon())*
        && <$last>::produces_epsilon()
    }}
}

macro_rules! check_left_recursive_impl {
    ($stack:ident, $set:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        let t = Self::type_id();
        if $set.contains(&t) {
            let mut stack = $stack.clone();
            stack.push(Self::debug().into_owned());
        return Err(LL1Error::LeftRecursion(stack.join(" -> ")));
        }
        $stack.push(Self::debug().into_owned());
        $set.insert(t);
        if let Err(e) = <$elem1>::check_left_recursive($stack, $set) {
            $stack.pop();
            $set.remove(&t);
            return Err(e);
        }

        let mut temp_stack = Vec::new();
        let mut temp_set = BTreeSet::new();

        let (cur_stack, cur_set, cur_is_main) = if <$elem1>::produces_epsilon() {
            ($stack, $set, true)
        } else {
            $stack.pop();
            $set.remove(&t);
            (&mut temp_stack, &mut temp_set, false)
        };

        $(
            if let Err(e) = <$elem>::check_left_recursive(cur_stack, cur_set) {
                if cur_is_main {
                    cur_stack.pop();
                    cur_set.remove(&t);
                }
                return Err(e);
            }
            let (cur_stack, cur_set, cur_is_main) = if <$elem>::produces_epsilon() {
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

        let check = <$last>::check_left_recursive(cur_stack, cur_set);
        if cur_is_main {
            cur_stack.pop();
            cur_set.remove(&t);
        }
        check
    }}
}

macro_rules! build_first_impl {
    ($builder:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        $builder.build_recursive::<$elem1>();
        $(
            $builder.build_recursive::<$elem>();
        )*
        $builder.build_recursive::<$last>();
        $builder.build_for_sequence(Self::type_id(), &[<$elem1>::type_id(), $(<$elem>::type_id()),* <$last>::type_id()]);
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
                    let terminals = current_check.intersection_terminal_minus_epsilon(next_first).into_iter().join(", ");
                    return Err(LL1Error::FirstFollowStringConflict(self_symbol, current_symbol, next_symbol, terminals));
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
                    let terminals = current_check.intersection_terminal_minus_epsilon(next_first).into_iter().join(", ");
                    return Err(LL1Error::FirstFollowStringConflict(self_symbol, current_symbol, next_symbol, terminals));
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

macro_rules! build_parsing_impl {
    ($seen:ident, $parsing:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        if $seen.insert(Self::type_id()) {
            <$elem1>::build_parsing($seen, $parsing);
            $(
                <$elem>::build_parsing($seen, $parsing);
            )*
            <$last>::build_parsing($seen, $parsing);
        }
    }}
}

macro_rules! try_parse_ast_impl {
    ($parser:ident, $meta:ident, $elem1:ty, $($elem:ty),* | $last:ty) => {{
        let token = $parser.peek_token_src();
        let t = Self::type_id();
        let first = $meta.first.get(&t);
        if !first.contains(token) {
            return AstResult::Panic(vec![$parser.expecting(first.clone())]);
        }
        let mut errors = Vec::new();
        let result = (
            match <$elem1>::try_parse_ast($parser, $meta) {
                AstResult::Success(x) => x,
                AstResult::Recovered(x, e) => { errors.extend(e); x },
                AstResult::Panic(e) => {
                    errors.extend(e);
                    return AstResult::Panic(errors);
                }
            },  $(

                match <$elem>::try_parse_ast($parser, $meta) {
                    AstResult::Success(x) => x,
                    AstResult::Recovered(x, e) => { errors.extend(e); x },
                    AstResult::Panic(e) => {
                        errors.extend(e);
                        return AstResult::Panic(errors);
                    }
                },

            ),*
            match <$last>::try_parse_ast($parser, $meta) {
                AstResult::Success(x) => x,
                AstResult::Recovered(x, e) => { errors.extend(e); x },
                AstResult::Panic(e) => {
                    errors.extend(e);
                    return AstResult::Panic(errors);
                }
            }
        );

        if errors.is_empty() {
            AstResult::Success(result)
        } else {
            AstResult::Recovered(result, errors)
        }
    }}
}

macro_rules! tuple_impl {
    ($elem1:ty, $($elem:ty),* | $last:ty) => {{
        impl<$elem1: SyntaxTree,
        $($elem: SyntaxTree<T=$elem1::T>),*>
        $last: SyntaxTree<T=$elem1::T>> SyntaxTree for ($elem1, $($elem),* $last) {
            type T = $elem1::T;
            type AST = ($elem1::AST, $($elem::AST),* $last::AST);

            #[inline]
            fn produces_epsilon() -> bool {
                produces_epsilon_impl!($elem1, $($elem),* | $last)
            }
        }
    }}
}


impl<A: SyntaxTree, B: SyntaxTree<T=A::T>> SyntaxTree for (A, B) {
    type T = A::T;
    type AST = (A::AST, B::AST);

    #[inline]
    fn produces_epsilon() -> bool {
        produces_epsilon_impl!(A,  | B)
    }

    #[inline]
    fn check_left_recursive(stack: &mut Vec<String>, set: &mut BTreeSet<TypeId>) -> Result<(), LL1Error> {
        check_left_recursive_impl!(stack, set, A, | B)
    }

    #[inline]
    fn build_first(builder: &mut FirstBuilder<Self::T>) {
        build_first_impl!(builder, A,  | B)
    }

    #[inline]
    fn check_first_conflict_recursive(seen: &mut BTreeSet<TypeId>, first: &First<Self::T>) -> Result<(), LL1Error> {
        check_first_conflict_impl!(seen, first, A,  | B)
    }

    #[inline]
    fn build_follow(builder: &mut FollowBuilder<Self::T>) {
        build_first_impl!(builder, A,  | B)
    }

    #[inline]
    fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
        build_parsing_impl!(seen, parsing, A,  | B)
    }

    #[inline]
    fn check_first_follow_conflict_recursive(seen: &mut BTreeSet<TypeId>, first: &First<Self::T>, follow: &Follow<Self::T>) -> Result<(), LL1Error> {
        check_first_follow_conflict_impl!(seen, first, follow, A,  | B)
    }

    #[inline]
    fn try_parse_ast<'s>( parser: &mut Parser<'s, Self::T>, meta: &RootMetadata<Self::T>,) -> AstResult<Self::T, Self::AST> {
        try_parse_ast_impl!(parser, meta, A,  | B)
    }
}
