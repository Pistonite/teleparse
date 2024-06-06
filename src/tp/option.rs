//! optional syntax tree nodes ([`Option`], [`Exists`])
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::option::Option as StdOption;


use crate::parser::ParseTree;
use crate::syntax::{First, FirstBuilder, FirstRel, Follow, FollowBuilder, FollowRel, Jump, Metadata, Result as SynResult};
use crate::{AbstractSyntaxTree, GrammarError, Parser, Span, ToSpan};

use super::Node;

/// Node that stores an optional subtree `Option<T> => T | epsilon`
#[derive(Node, ToSpan)]
pub struct Option<T: ParseTree>(Node<StdOption<T>>);

impl<T: ParseTree> ParseTree for Option<T> {
    type AST = OptionAST<T::AST>;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        match ast {
            OptionAST::Some(ast) => {
                let t = T::from_ast(ast, parser);
                Node::new(t.span(), Some(t)).into()
            }
            OptionAST::None(span) => {
                Node::new(span, None).into()
            }
        }
    }
}

/// Node that stores if an optional subtree is parsed
#[derive(Node, ToSpan)]
pub struct Exists<T: ParseTree>(Node<bool>, PhantomData<T>);

impl<T: ParseTree> ParseTree for Exists<T> {
    type AST = OptionAST<T::AST>;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        match ast {
            OptionAST::Some(ast) => {
                let t = T::from_ast(ast, parser);
                Node::new(t.span(), true).into()
            }
            OptionAST::None(span) => {
                Node::new(span, false).into()
            }
        }
    }
}

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
        if builder.visit(t) {
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
    fn check_left_recursive(stack: &mut Vec<String>, set: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError> {
        AST::check_left_recursive(stack ,set, first)
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
            return SynResult::Success(Self::None(parser.current_span()));
        }
        let first = meta.first.get(&AST::type_id());
        if !first.contains(token) {
            // produces epsilon
            return SynResult::Success(Self::None(parser.current_span()));
        }

        // if parse fails, delay to parent to panic
        match AST::parse_ast(parser, meta) {
            SynResult::Success(ast) => {
                SynResult::Success(Self::Some(ast))
            },
            SynResult::Recovered(ast, error) =>
                SynResult::Recovered(Self::Some(ast), error),
            SynResult::Panic(error) =>
                SynResult::Recovered(Self::None(parser.current_span()), error),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn use_optional_as_option() {
    //     let o = Option(Node::new(Span::new(0, 0), Some(42)));
    //     assert_eq!(o.as_ref().copied(), Some(42));
    //     let opt: &Option<i32> = &o;
    //     assert_eq!(opt.as_ref().copied(), Some(42));
    // }
    // #[test]
    // fn use_exists() {
    //     let e = Exists::<String>::new(Span::new(0, 0), true);
    //     assert!(e.exists());
    // }
}
