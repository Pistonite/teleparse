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
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Option<T: ParseTree>(Node<StdOption<T>>);

impl<T: std::fmt::Debug + ParseTree> std::fmt::Debug for Option<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0.value {
            Some(node) => {
                f.debug_tuple("Some").field(&node).finish()
            }
            None => {
                f.debug_tuple("None").field(&self.0.span).finish()
            }
        }
    }
}

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

/// Node that stores if an optional subtree is produced
#[derive(Node, ToSpan, Clone, PartialEq)]
pub struct Exists<T: ParseTree>(Node<bool>, PhantomData<T>);

impl<T: ParseTree> std::fmt::Debug for Exists<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

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


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::GrammarError;
    use crate::ParseTree;
    use crate::tp::Node;

    use crate::lex::Token;
    use crate::test::prelude::*;
    use crate::test::MathTokenType as T;
    use crate::test::{Ident, OpAdd};

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct OptIdent(super::Option<Ident>);

    #[test]
    fn test_none() {
        let t = OptIdent::parse("+").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "None(0)");
        assert_eq!(t, OptIdent(Node::new(0..0, None).into()));
    }

    #[test]
    fn test_some() {
        let t = OptIdent::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "Some(token Ident(0..1))");
        assert_eq!(t, OptIdent(Node::new(0..1, Some(
            Ident(Token::new(0..1, T::Ident))
        )).into()));
    }

    #[test]
    fn test_use_as_option() {
        let t = OptIdent::parse("+").unwrap().unwrap();
        assert!(t.0.is_none());
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    struct Seq(super::Option<OpAdd>, OpAdd);

    #[test]
    fn test_seq_not_ll1() {
        assert_not_ll1!(Seq, GrammarError::FirstFollowSeqConflict(
            "Seq".to_string(),
            "Option<OpAdd>".to_string(),
            "OpAdd".to_string(),
            "\"+\"".to_string()
        ));
    }

    #[derive_syntax]
    #[teleparse(root, no_test)]
    struct Nested(super::Option<super::Option<Ident>>);

    #[test]
    fn test_nested_not_ll1() {
        assert_not_ll1!(Nested, GrammarError::FirstFirstConflict(
            "Option<Option<Ident>>".to_string(),
            "Option<Ident>".to_string(),
            "<epsilon>".to_string(),
        ));
    }

    #[derive_syntax]
    #[teleparse(root)]
    #[derive(Debug, PartialEq)]
    struct ExistIdent(super::Exists<Ident>);

    #[test]
    fn parse_exist() {
        let t = ExistIdent::parse("a").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0..1 => true");
        assert_eq!(t, ExistIdent(Node::new(0..1, true).into()));
        assert_eq!(*t.0, true);

        let t = ExistIdent::parse("+").unwrap().unwrap();
        let t_str = format!("{:?}", t.0);
        assert_eq!(t_str, "0 => false");
        assert_eq!(t, ExistIdent(Node::new(0..0, false).into()));
        assert_eq!(*t.0, false);
    }
}
