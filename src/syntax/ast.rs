use std::{any::TypeId, marker::PhantomData};
use std::borrow::Cow;
use std::collections::BTreeSet;

use itertools::Itertools;

use crate::{GrammarError, Lexicon, Pos, Span, ToSpan};

use super::{First, FirstBuilder, Follow, FollowBuilder, MetadataBuilder, Jump};

pub struct Epsilon<L: Lexicon + 'static>(PhantomData<L>);

impl<L: Lexicon + 'static> Production for Epsilon<L> {
    type L = L;
    fn debug() -> Cow<'static, str>{
        Cow::Borrowed("epsilon")
    }
    fn register(meta: &mut MetadataBuilder<Self::L>) {
        let t = Self::id();
        if meta.visit(t, ||Self::debug().into_owned()) {
            meta.add_epsilon(t);
        }
    }
}

/// An AST node
///
/// See [module-level documentation](super) for more information.
pub trait Production: 'static {
    /// The token type of the AST node
    type L: Lexicon + 'static;

    /// Get the unique type id of the AST node,
    /// which represents one production in the grammar (multiple production in case of a
    /// union/enum)
    #[inline]
    fn id() -> TypeId {
        TypeId::of::<Self>()
    }

    /// Get the type name for the AST node for debugging
    #[inline]
    fn debug() -> Cow<'static, str>{
        Cow::Borrowed(std::any::type_name::<Self>())
    }

    fn register(meta: &mut MetadataBuilder<Self::L>);

    // /// Add the rules for this AST node (recursively) to the FIRST function builder.
    // ///
    // /// Note this may not terminate if the grammar is left-recursive
    // fn build_first(builder: &mut FirstBuilder<Self::L>);
    //
    // /// Check if the grammar at this AST node is left-recursive
    // ///
    // /// Left-recursion will lead to infinite recursion when parsing, so it is not allowed
    // fn check_left_recursive(seen: &mut BTreeSet<TypeId>, stack: &mut Vec<String>, set: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError>;
    //
    // /// Check for conflicts in the FIRST set of this AST node
    // fn check_first_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError>;
    //
    // /// Add the rules for this AST node (recursively) to the FOLLOW function builder.
    // fn build_follow(builder: &mut FollowBuilder<Self::L>);
    //
    // /// Check for conflicts in the FIRST and FOLLOW set of this AST node recursively
    // fn check_first_follow_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, follow: &Follow<Self::L>) -> Result<(), GrammarError>;
    //
    // /// Check if epsilon is in FIRST(t) and FIRST(t) intersect FOLLOW(t)
    // ///
    // /// This is used in derived AST implementations to check for conflicts
    // fn check_self_first_follow_conflict(first: &First<Self::L>, follow: &Follow<Self::L>) -> Result<(), GrammarError> {
    //     let t = Self::type_id();
    //     let first_set = first.get(&t);
    //     if !first_set.contains_epsilon() {
    //         return Ok(());
    //     }
    //     let follow_set = follow.get(&t);
    //     if follow_set.intersects_first(first_set) {
    //         let name = Self::debug().into_owned();
    //         let intersection = follow_set.intersection_repr_first(first_set).into_iter().join(", ");
    //         Err(GrammarError::FirstFollowConflict(name, intersection))
    //     } else {
    //         Ok(())
    //     }
    // }
    //
    // /// Recursively build the parsing table for this AST node.
    // ///
    // /// See [Predictive parsing table](`super::jump`) for more information.
    // fn build_jump(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, jump: &mut Jump<Self::L>);
}


#[macro_export]
#[doc(hidden)]
macro_rules! production_passthrough {
    ($P:ty) => {
        type L=<$P as $crate::syntax::Production>::L;
        #[inline]
        fn id() -> ::core::any::TypeId {
            <$P>::id()
        }
        #[inline]
        fn debug() -> ::std::borrow::Cow<'static, str> {
            <$P>::debug()
        }
        #[inline]
        fn register(meta: &mut $crate::syntax::MetadataBuilder<Self::L>) {
            <$P>::register(meta)
        }
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! register_sequence {
    ($meta:ident, $($T:ty),*) => {{
        let t = Self::id();
        if $meta.visit(t, ||Self::debug().into_owned()) {
            $meta.add_sequence(t, &[ $(<$T>::id()),* ]);
            $(<$T>::register($meta);)*
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! register_union {
    ($meta:ident, $($T:ty),*) => {{
        let t = Self::id();
        if $meta.visit(t, ||Self::debug().into_owned()) {
            $meta.add_union(t, &[ $(<$T>::id()),* ]);
            $(<$T>::register($meta);)*
        }
    }};
}
