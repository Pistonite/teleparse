use std::{any::TypeId, collections::{BTreeSet, HashMap, HashSet}, rc::Rc, sync::{Arc, OnceLock}};
use std::borrow::Cow;

use crate::{
    table::{SyntaxTreeTable, LitTable, TermSet}, Parser, SyntaxResult, ToSpan, Token, TokenType, TokenTypeNoCtx
};

type CtxOf<T> = <T as TokenType>::Ctx;

pub trait SyntaxTree: Sized + ToSpan {
    type T: TokenType;
    type AST: ToSpan;

    fn build_start_table(s_table: &mut SyntaxTreeTable<Self::T>, lits: &mut LitTable);

    fn build_follow_table<'s>(
        s_table: &'s SyntaxTreeTable<Self::T>, 
        f_table: &mut SyntaxTreeTable<Self::T>,
        follows: &TermSet<Self::T>
        ) -> Cow<'s, TermSet<Self::T>>;

    // fn span_of(ast: &Self::AST) -> Span;
    //
    // fn start_set() -> HashSet<Start<Self::T>>;

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
