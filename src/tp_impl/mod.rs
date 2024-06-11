//! Implementation for standard library types and utility types
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::marker::PhantomData;

use deref_derive::{Deref, DerefMut};

use crate::parser::ParseTree;
use crate::syntax::{First, FirstBuilder, FirstRel, Follow, FollowBuilder, FollowRel, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, Parser, Span, ToSpan};


pub mod boxed;
// pub mod blanket;
pub mod iter;
pub mod option;
pub mod string;
pub mod tuple;

#[derive(Deref, DerefMut, ToSpan, Clone, PartialEq)]
pub struct Node<T> {
    pub span: Span,
    #[deref]
    pub value: T,
}
pub use teleparse_macros::Node;

impl<T> Node<T> {
    pub fn new<S: Into<Span>>(span: S, value: T) -> Self {
        Self { span: span.into(), value }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self.span)?;
        self.value.fmt(f)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! ast_passthrough {
    ($ast:ty) => {
        type L=<$ast as $crate::syntax::AbstractSyntaxTree>::L;
        fn type_id() -> ::core::any::TypeId {
            <$ast>::type_id()
        }
        fn debug() -> ::std::borrow::Cow<'static, str> {
            <$ast>::debug()
        }
        fn build_first(builder: &mut $crate::syntax::first::FirstBuilder<Self::L>) {
            <$ast>::build_first(builder)
        }
        fn check_left_recursive(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            stack: &mut ::std::vec::Vec<::std::string::String>,
            set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_left_recursive(seen, stack, set, first)
        }
        fn check_first_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>, 
            first: &$crate::syntax::first::First<Self::L>
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_first_conflict(seen, first)
        }
        fn build_follow(
            builder: &mut $crate::syntax::follow::FollowBuilder<Self::L>
        ) {
            <$ast>::build_follow(builder)
        }
        fn check_first_follow_conflict(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
            follow: &$crate::syntax::follow::Follow<Self::L>,
        ) -> ::core::result::Result<(), $crate::GrammarError> {
            <$ast>::check_first_follow_conflict(seen, first, follow)
        }
        fn build_jump(
            seen: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
            first: &$crate::syntax::first::First<Self::L>,
            jump: &mut $crate::syntax::jump::Jump<Self::L>
        ) {
            <$ast>::build_jump(seen, first, jump)
        }
    };
}
pub(crate) use ast_passthrough;

#[doc(hidden)]
#[derive(ToSpan)]
pub enum OptionAST<T: AbstractSyntaxTree> {
    Some(T),
    None(Span),
}

impl<AST: AbstractSyntaxTree> AbstractSyntaxTree for OptionAST<AST> {
    type L = AST::L;

    #[inline]
    fn debug() -> Cow<'static, str> {
        Cow::Owned(format!("Option<{}>", AST::debug()))
    }

    #[inline]
    fn build_first(builder: &mut FirstBuilder<Self::L>) {
        let t = Self::type_id();
        if builder.visit(t, &Self::debug()) {
            // recursive build
            AST::build_first(builder);
            let inner = AST::type_id();
            // Option<T> => T
            // usually we need to check if T can be empty
            // but since epsilon is added below anyway, we don't need to check
            builder.add(FirstRel::union_minus_epsilon(t, inner));
            // Option<T> => epsilon
            builder.add(FirstRel::insert_epsilon(t));
        }

    }

    #[inline]
    fn check_left_recursive(seen: &mut BTreeSet<TypeId>, stack: &mut Vec<String>, set: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError> {
        AST::check_left_recursive(seen, stack ,set, first)
    }

    #[inline]
    fn check_first_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError> {
        if !seen.insert(Self::type_id()) {
            return Ok(());
        }
        // Self -> Inner | e
        // Collides if Inner contains e
        if first.get(&AST::type_id()).contains_epsilon() {
            let type_name = Self::debug().into_owned();
            let inner_name = AST::debug().into_owned();
            return Err(GrammarError::FirstFirstConflict(
                type_name, inner_name, "<epsilon>".to_string()));
        }
        AST::check_first_conflict(seen, first)
    }

    #[inline]
    fn build_follow(builder: &mut FollowBuilder<Self::L>) {
        let t = Self::type_id();
        if builder.visit(t) {
            // recursive build
            AST::build_follow(builder);

            let inner = AST::type_id();
            // Option<T> => T
            builder.add(FollowRel::union_follow(inner, t));
        }
    }

    #[inline]
    fn check_first_follow_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, follow: &Follow<Self::L>) -> Result<(), GrammarError> {
        if !seen.insert(Self::type_id()) {
            return Ok(());
        }
        Self::check_self_first_follow_conflict(first, follow)?;
        AST::check_first_follow_conflict(seen, first, follow)
    }

    #[inline]
    fn build_jump(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, jump: &mut Jump<Self::L>) {
        if seen.insert(Self::type_id()) {
            AST::build_jump(seen, first, jump);
        }
    }

    fn parse_ast<'s>(
        parser: &mut Parser<'s, Self::L>,
        meta: &Metadata<Self::L>,
    ) -> SynResult<Self, Self::L> {
        let token = parser.peek_token_src();
        if token.is_none() {
            // produces epsilon
            return SynResult::Success(Self::None(parser.current_span_empty()));
        }
        let first = meta.first.get(&AST::type_id());
        if !first.contains(token) {
            // produces epsilon
            return SynResult::Success(Self::None(parser.current_span_empty()));
        }

        // if parse fails, delay to parent to panic
        match AST::parse_ast(parser, meta) {
            SynResult::Success(ast) => {
                SynResult::Success(Self::Some(ast))
            },
            SynResult::Recovered(ast, error) =>
                SynResult::Recovered(Self::Some(ast), error),
            SynResult::Panic(error) =>
                SynResult::Recovered(Self::None(parser.current_span_empty()), error),
        }
    }
}
