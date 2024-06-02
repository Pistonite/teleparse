use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

// use crate::table::{SyntaxTreeTable, LitTable, FirstSet};
use crate::{ToSpan, Span, TokenType};
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

    #[inline]
    fn debug_type(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<Self::AST>())
    }

    fn produces_epsilon() -> bool;

    fn check_left_recursive(stack: &mut Vec<TypeId>, set: &mut BTreeSet<TypeId>) -> Option<Vec<TypeId>>;
    
    fn build_first(builder: &mut FirstBuilder<Self::T>);
    
    fn check_conflict(first: &First<Self::T>) -> bool;

    fn build_follow(builder: &mut FollowBuilder<Self::T>);

    // /// Attempt to parse one AST node
    // ///
    // /// This is a recursive API that should be derived instead of implemented
    // fn try_parse_ast<'s>(
    //     parser: &mut Parser<'s, Self::T>, 
    //     // f_table: &SyntaxTreeTable<Self::T>,
    //     should_recover: bool
    // ) -> SyntaxResult<Self::T, Self::AST>;

    // /// Transform the parsed AST node into the final tree node
    // ///
    // /// This is a recursive API that should be derived instead of implemented
    // fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self;
}
