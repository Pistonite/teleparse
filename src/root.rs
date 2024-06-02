use crate::{table::{first::First, follow::Follow, parsing::Parsing}, SyntaxTree, TokenType, TokenTypeNoCtx};

/// Helper trait to define functions on Root
type CtxOf<T> = <T as TokenType>::Ctx;

pub trait Root: SyntaxTree + 'static {
    // fn parse_with_context( source: &str, context: CtxOf<Self::T>) -> (Option<Self>, CtxOf<Self::T>) {
    //     let mut parser = Self::T::parser_with_context(source, context);
    //     let result = parser.once();
    //     (result, parser.context)
    // }
    //
    // fn parse_all_with_context( source: &str, context: CtxOf<Self::T>) -> (Vec<Self>, CtxOf<Self::T>) {
    //     let mut parser = Self::T::parser_with_context(source, context);
    //     let result = parser.parse_all();
    //     (result, parser.context)
    // }

    fn root_metadata() -> &'static Result<RootMetadata<Self::T>, LL1Error>;
}

pub struct RootMetadata<T: TokenType>{
    pub first: First<T>,
    pub follow: Follow<T>,
    pub parsing: Parsing<T>,
}

#[derive(Debug, thiserror::Error)]
pub enum LL1Error {
    #[error("Left recursion detected in the grammar! Stack: {0}")]
    LeftRecursion(String),
    #[error("The non-terminal `{0}` has a FIRST/FIRST conflict producing `{1}`/`{2}`. The conflicting terminals are: {3}")]
    FirstFirstConflict(String, String, String, String),
    #[error("The non-terminal `{0}` has a FIRST/FOLLOW conflict producing `{1}`/`{2}`. The conflicting terminals are: {3}")]
    FirstFollowStringConflict(String, String, String, String),
    #[error("The non-terminal `{0}` has conflict in its FIRST and FOLLOW sets. The conflicting terminals are: {1}")]
    FirstFollowConflict(String, String),
}

pub trait RootNoCtx: Root {
    // fn parse(source: &str) -> Option<Self>;
    //
    // fn parse_all(source: &str) -> Vec<Self>;
}

impl<T: TokenTypeNoCtx, AST, ST: Root<T=T, AST=AST>> RootNoCtx for ST {
    // #[inline]
    // fn parse(source: &str) -> Option<Self> {
    //     let (result, _) = Self::parse_with_context(source, ());
    //     result
    // }
    //
    // #[inline]
    // fn parse_all(source: &str) -> Vec<Self> {
    //     let (result, _) = Self::parse_all_with_context(source, ());
    //     result
    // }
}

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

#[macro_export]
macro_rules! assert_ll1 {
    ($root:ty) => {{
        let r = <$root as $crate::root::Root>::root_metadata();
        assert!(r.is_ok(), "{} is not LL(1): {}", <$root as $crate::SyntaxTree>::debug(), r.unwrap_err());
    }}
}

/// Internal implementation for deriving syntax tree [`Root`]
#[macro_export]
macro_rules! derive_root_impl {
    ($ident:ty) => {
        fn root_metadata() -> &'static ::std::result::Result<$crate::root::RootMetadata<Self>, $crate::root::LL1Error>{
            static METADATA: ::std::sync::OnceLock<::std::result::Result<$crate::root::RootMetadata<$ident>, $crate::root::LL1Error>> = std::sync::OnceLock::new();
            METADATA.get_or_init(|| {
                let mut stack = std::vec::Vec::new();
                let mut seen = std::collections::BTreeSet::new();
                Self::check_left_recursive(&mut stack, &mut seen)?;

                let mut first = $crate::table::first::FirstBuilder::new();
                Self::build_first(&mut first);
                let first = first.build();
                seen.clear();
                Self::check_first_conflict(&mut seen, &first)?;
                
                let mut follow = $crate::table::follow::FollowBuilder::new(first);
                Self::build_follow(&mut follow);
                let (first, follow) = follow.build();
                seen.clear();
                Self::check_first_follow_conflict(&mut seen, &first, &follow)?;

                let mut parsing = $crate::table::parsing::Parsing::new();
                seen.clear();
                Self::build_parsing(&mut seen, &mut parsing);

                Ok($crate::root::RootMetadata {
                    first_table,
                    follow_table,
                    parsing
                })
            })
        }
    }
}
