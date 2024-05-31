use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::table::{SyntaxTreeTable, LitTable, TermSet};
use crate::{Parser, SyntaxResult, ToSpan, TokenType};


pub trait SyntaxTree: Sized + ToSpan {
    type T: TokenType;
    type AST: ToSpan + 'static;

    /// Get the unique type id of the AST node
    ///
    /// Note that multiple Syntax tree implementation could have the same AST type,
    /// and thus the same type id. For example, [`Quote`](crate::tp::Quote) and [`Parse`](crate::tp::Parse)
    fn type_id() -> TypeId {
        TypeId::of::<Self::AST>()
    }

    fn can_be_empty() -> bool;

    fn check_left_recursive(stack: &mut Vec<TypeId>, set: &mut BTreeSet<TypeId>) -> bool;

    fn build_first_table(
        first: &mut SyntaxTreeTable<Self::T>, 
        lits: &mut LitTable);

    fn has_first_collision(first: &SyntaxTreeTable<Self::T>) -> bool;

    fn build_follow_table(
        first: &SyntaxTreeTable<Self::T>, 
        follow: &mut SyntaxTreeTable<Self::T>,
        // follows: &TermSet<Self::T>
        ) -> bool;

    /// Attempt to parse one AST node
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn try_parse_ast<'s>(
        parser: &mut Parser<'s, Self::T>, 
        f_table: &SyntaxTreeTable<Self::T>,
        should_recover: bool
    ) -> SyntaxResult<Self::T, Self::AST>;

    /// Transform the parsed AST node into the final tree node
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self;
}
