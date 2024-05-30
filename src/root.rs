use crate::{SyntaxTree, TokenType, TokenTypeNoCtx};
use crate::table::SyntaxTreeTable;

/// Helper trait to define functions on Root
type CtxOf<T> = <T as TokenType>::Ctx;

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
}

pub struct RootMetadata<ST: Root>{
    pub is_ll1: bool,
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

/// Marker trait for LL1 grammar
pub trait LL1 {}

/// Macro to derive [`Root`] for generated Terminal types.
/// This is used internally in tests and examples. Library users
/// should simply `#[derive(Root)]` instead.
#[macro_export]
macro_rules! derive_root {
    ($ident:ident) => {
        #[automatically_derived]
        impl $crate::root::Root for $ident {
            $crate::derive_root_impl!($ident);
        }
    }
}

/// Internal implementation for deriving syntax tree [`Root`]
#[macro_export]
macro_rules! derive_root_impl {
    ($ident:ident) => {
        fn root_metadata() -> &'static $crate::root::RootMetadata<Self> {
            static METADATA: std::sync::OnceLock<$crate::root::RootMetadata<$ident>> = std::sync::OnceLock::new();
            METADATA.get_or_init(|| {
                let mut start_table = $crate::table::SyntaxTreeTable::default();
                let mut lit_table = $crate::table::LitTable::default();
                let mut no_first_first_collision = Self::build_start_table(&mut start_table, &mut lit_table);
                let mut follow_table = $crate::table::SyntaxTreeTable::default();
                // the root's follow table is EOF
                let mut follows = $crate::table::TermSet::default();
                follows.insert_eof();

                let (_, no_first_follow_collsion) = Self::build_follow_table(&start_table, &mut follow_table, &follows);
                $crate::root::RootMetadata {
                    is_ll1: no_first_first_collision && no_first_follow_collsion,
                    start_table,
                    follow_table,
                }
            })
        }
    }
}
