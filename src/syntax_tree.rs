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
        f_table: &SyntaxTreeTable<Self::T>) -> SyntaxResult<Self::T, Self::AST>;

    /// Transform the parsed AST node into the final tree node
    ///
    /// This is a recursive API that should be derived instead of implemented
    fn into_parse_tree<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::T>) -> Self;
}

pub trait Root: SyntaxTree + 'static {
    fn parse_with_context( source: &str, context: CtxOf<Self::T>) -> (Option<Self>, CtxOf<Self::T>) {
        let mut parser = Self::T::parser_with_context(source, context);
        let result = parser.once();
        (result, parser.context)
    }

    fn parse_all_with_context( source: &str, context: CtxOf<Self::T>) -> (Vec<Self>, CtxOf<Self::T>) {
        let mut parser = Self::T::parser_with_context(source, context);
        let result = parser.parse_all();
        (result, parser.context)
    }

    fn root_metadata() -> &'static RootMetadata<Self>;
    // this needs to be derived
    // {
    //     static ONCE: OnceLock<RootMetadata<Self::T>> = OnceLock::new();
    //     ONCE.get_or_init(|| {
    //         let mut start_table = SyntaxTreeTable::default();
    //         let mut lit_table = LitTable::default();
    //         Self::build_start_table(&mut start_table, &mut lit_table);
    //         let mut follow_table = SyntaxTreeTable::default();
    //         // the root's follow table is EOF
    //         let mut follows = TermSet::default();
    //         follows.insert_eof();
    //         Self::build_follow_table(&start_table, &mut follow_table, &follows);
    //         RootMetadata {
    //             start_table,
    //             follow_table,
    //         }
    //     })
    // }
}

pub struct RootMetadata<ST: Root>{
    pub start_table: SyntaxTreeTable<ST::T>,
    pub follow_table: SyntaxTreeTable<ST::T>,
}

pub trait RootNoCtx: Root {
    fn parse(source: &str) -> Option<Self>;
    
    fn parse_all(source: &str) -> Vec<Self>;
}

impl<T: TokenTypeNoCtx, AST, ST: Root<T=T, AST=AST>> RootNoCtx for ST {
    #[inline]
    fn parse(source: &str) -> Option<Self> {
        let (result, _) = Self::parse_with_context(source, ());
        result
    }
    
    #[inline]
    fn parse_all(source: &str) -> Vec<Self> {
        let (result, _) = Self::parse_all_with_context(source, ());
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenRef<'s, T: TokenType> {
    ty: T,
    lit: &'s str,
}

