// use crate::table::first::First;
// use crate::table::follow::Follow;
// use crate::table::parsing::Parsing;
// use crate::{TokenType, TokenTypeNoCtx};
//
// use super::ParseTree;
//
// macro_rules! Type {
//     (T) => {
//         <Self::AST as $crate::syntax::AbstractSyntaxTree>::T
//     };
//     (Ctx) => {
//         <<Self::AST as $crate::syntax::AbstractSyntaxTree>::T as $crate::TokenType>::Ctx
//     };
// }

use crate::{GrammarError, Lexicon};

use super::{AbstractSyntaxTree, First, Follow, Jump};

/// The root of the Abstract Syntax Tree (AST) for a grammar
///
/// Deriving this trait provides static storage of the metadata of the grammar such as 
/// the FIRST and FOLLOW functions.
pub trait AbstractSyntaxRoot: AbstractSyntaxTree {
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

    /// Get the static metadata
    fn metadata() -> &'static Result<Metadata<Self::L>, GrammarError>;
}

pub struct Metadata<L: Lexicon>{
    pub first: First<L>,
    pub follow: Follow<L>,
    pub jump: Jump<L>,
}
//
//
// // pub trait RootNoCtx: Root {
// //     // fn parse(source: &str) -> Option<Self>;
// //     //
// //     // fn parse_all(source: &str) -> Vec<Self>;
// // }
// //
// // impl<T: TokenTypeNoCtx, AST, ST: Root<T=T, AST=AST>> RootNoCtx for ST {
// //     // #[inline]
// //     // fn parse(source: &str) -> Option<Self> {
// //     //     let (result, _) = Self::parse_with_context(source, ());
// //     //     result
// //     // }
// //     //
// //     // #[inline]
// //     // fn parse_all(source: &str) -> Vec<Self> {
// //     //     let (result, _) = Self::parse_all_with_context(source, ());
// //     //     result
// //     // }
// // }
//
// /// Macro to derive [`Root`] for generated Terminal types.
// /// This is used internally in tests and examples. Library users
// /// should simply `#[derive(Root)]` instead.
// #[macro_export]
// macro_rules! derive_root {
//     ($ident:ident) => {
//         #[automatically_derived]
//         impl $crate::root::Root for $ident {
//             $crate::derive_root_impl!($ident);
//         }
//     }
// }
//
// #[macro_export]
// macro_rules! assert_ll1 {
//     ($root:ty) => {{
//         let r = <$root as $crate::root::Root>::root_metadata();
//         assert!(r.is_ok(), "{} is not LL(1): {}", <$root as $crate::SyntaxTree>::debug(), r.unwrap_err());
//     }}
// }
//
// /// Internal implementation for deriving syntax tree [`Root`]
// #[macro_export]
// macro_rules! derive_root_impl {
//     ($ident:ty) => {
//         fn root_metadata() -> &'static ::std::result::Result<$crate::root::RootMetadata<Self>, $crate::root::LL1Error>{
//             static METADATA: ::std::sync::OnceLock<::std::result::Result<$crate::root::RootMetadata<$ident>, $crate::root::LL1Error>> = std::sync::OnceLock::new();
//             METADATA.get_or_init(|| {
//                 let mut stack = std::vec::Vec::new();
//                 let mut seen = std::collections::BTreeSet::new();
//                 Self::check_left_recursive(&mut stack, &mut seen)?;
//
//                 let mut first = $crate::table::first::FirstBuilder::new();
//                 Self::build_first(&mut first);
//                 let first = first.build();
//                 seen.clear();
//                 Self::check_first_conflict(&mut seen, &first)?;
//                
//                 let mut follow = $crate::table::follow::FollowBuilder::new(first);
//                 Self::build_follow(&mut follow);
//                 let (first, follow) = follow.build();
//                 seen.clear();
//                 Self::check_first_follow_conflict(&mut seen, &first, &follow)?;
//
//                 let mut parsing = $crate::table::parsing::Parsing::new();
//                 seen.clear();
//                 Self::build_parsing(&mut seen, &mut parsing);
//
//                 Ok($crate::root::RootMetadata {
//                     first_table,
//                     follow_table,
//                     parsing
//                 })
//             })
//         }
//     }
// }
