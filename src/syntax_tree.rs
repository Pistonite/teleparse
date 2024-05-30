use std::borrow::Cow;

use crate::table::{SyntaxTreeTable, LitTable, TermSet};
use crate::{Parser, SyntaxResult, ToSpan, TokenType};


pub trait SyntaxTree: Sized + ToSpan {
    type T: TokenType;
    type AST: ToSpan;

    fn build_start_table(
        s_table: &mut SyntaxTreeTable<Self::T>, 
        lits: &mut LitTable) -> bool; // is_ll1

    fn build_follow_table<'s>(
        s_table: &'s SyntaxTreeTable<Self::T>, 
        f_table: &mut SyntaxTreeTable<Self::T>,
        follows: &TermSet<Self::T>
        ) -> (Cow<'s, TermSet<Self::T>>, bool); // is_ll1

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
