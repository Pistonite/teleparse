use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use itertools::Itertools;

use crate::parser::Parser;
use crate::root::RootMetadata;
use crate::table::parsing::Parsing;
// use crate::table::{SyntaxTreeTable, LitTable, FirstSet};
use crate::{AstResult, LL1Error, Span, ToSpan, TokenType};
use crate::table::first::{First, FirstBuilder};
use crate::table::follow::{Follow, FollowBuilder};


pub trait SyntaxTree: Sized + ToSpan {
    type T: TokenType;
    type AST: ToSpan + 'static;

    /// Get the unique type id of the AST node
    ///
    /// Note that multiple Syntax tree implementation could have the same AST type,
    /// and thus the same type id. For example, [`Quote`](crate::tp::Quote) and [`Parse`](crate::tp::Parse)
    #[inline]
    fn type_id() -> TypeId {
        TypeId::of::<Self::AST>()
    }

    /// Get the type name for the AST node for debugging
    #[inline]
    fn debug() -> Cow<'static, str>{
        Cow::Borrowed(std::any::type_name::<Self::AST>())
    }

    fn produces_epsilon() -> bool;

    fn check_left_recursive(stack: &mut Vec<String>, set: &mut BTreeSet<TypeId>) -> Result<(), LL1Error>;
    
    fn build_first(builder: &mut FirstBuilder<Self::T>);
    
    #[inline]
    fn check_first_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::T>) -> Result<(), LL1Error> {
        if seen.insert(Self::type_id()) {
            Self::check_first_conflict_recursive(seen, first)
        } else {
            Ok(())
        }
    }
    fn check_first_conflict_recursive(seen: &mut BTreeSet<TypeId>, first: &First<Self::T>) -> Result<(), LL1Error>;

    fn build_follow(builder: &mut FollowBuilder<Self::T>);

    fn check_first_follow_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::T>, follow: &Follow<Self::T>) -> Result<(), LL1Error> {
        let t = Self::type_id();
        if !seen.insert(t) {
            return Ok(());
        }
        Self::check_first_follow_conflict_recursive(seen, first, follow)?;
        let first_set = first.get(&t);
        if first_set.contains_epsilon() {
            let follow_set = follow.get(&t);
            if follow_set.intersects_first(first_set) {
                let terminals = follow_set.intersection_terminal_first(first_set);
                let terminals = terminals.into_iter().join(", ");
                let type_name = Self::debug();
                return Err(LL1Error::FirstFollowConflict(type_name.into_owned(),
                    terminals));
            }
        }

        Ok(())
    }

    fn check_first_follow_conflict_recursive(seen: &mut BTreeSet<TypeId>, first: &First<Self::T>, follow: &Follow<Self::T>) -> Result<(), LL1Error>;

    fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>);

    fn try_parse_ast<'s>(
        parser: &mut Parser<'s, Self::T>, 
        meta: &RootMetadata<Self::T>,
    ) -> AstResult<Self::T, Self::AST>;

    // /// Transform the parsed AST node into the final tree node
    // ///
    // /// This is a recursive API that should be derived instead of implemented
    // fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self;
}
