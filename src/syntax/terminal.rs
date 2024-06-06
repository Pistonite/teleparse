use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::{GrammarError, Lexicon, Parser, ToSpan};
use crate::lex::Token;

use super::{AbstractSyntaxTree, First, FirstBuilder, FirstRel, Follow, FollowBuilder, Jump, Metadata};

/// A terminal AST node that produces a single token that optionally matches a specific literal
/// constant
pub trait Terminal: ToSpan + From<Token<Self::L>> {
    type L: Lexicon;

    /// The identifier of the terminal used in debugging
    fn ident() -> & 'static str;

    /// The token type of the terminal
    fn token_type() -> Self::L;

    /// The literal constant that the terminal should match, if any
    fn match_literal() -> Option<&'static str>;
}

// impl<L: Lexicon, T: Terminal<L=L> + ToSpan + From<Token<L>> + 'static> AbstractSyntaxTree for T {
//     type L = L;
//     #[inline]
//
//     fn debug() -> Cow<'static, str> {
//         Cow::Borrowed(Self::ident())
//     }
//
//     #[inline]
//     fn build_first(builder: &mut FirstBuilder<Self::L>) {
//         let t = Self::type_id();
//         let expr = FirstRel::insert_token(t, Self::token_type(), Self::match_literal());
//         builder.add(expr);
//     }
//
//     #[inline]
//     fn check_left_recursive(_stack: &mut Vec<String>, _seen: &mut BTreeSet<TypeId>, _first: &First<Self::L>) -> Result<(), GrammarError> {
//         // a terminal has no recursive rules
//         Ok(())
//     }
//   
//   
//     /// Check for conflicts in the FIRST set of this AST node
//     #[inline]
//     fn check_first_conflict(_seen: &mut BTreeSet<TypeId>, _first: &First<Self::L>) -> Result<(), GrammarError> {
//         // a terminal has no recursive rules and therefore no conflicts
//         Ok(())
//     }
//
//     #[inline]
//     fn build_follow(_builder: &mut FollowBuilder<Self::L>) {
//         // no FOLLOW rules are produced from a terminal
//     }
//
//     #[inline]
//     fn check_first_follow_conflict(_seen: &mut BTreeSet<TypeId>, _first: &First<Self::L>, _follow: &Follow<Self::L>) -> Result<(), GrammarError> {
//         // terminals don't produce epsilon and therefore has no FIRST/FOLLOW conflict
//         Ok(())
//     }
//
//     #[inline]
//     fn build_jump(_seen: &mut BTreeSet<TypeId>, _first: &First<Self::L>, _jump: &mut Jump<Self::L>) {
//         // no parse table needed
//     }
//
//     /// Parse this AST node from the input stream
//     #[inline]
//     fn parse<'s>(
//         parser: &mut Parser<'s, Self::L>, 
//         meta: &Metadata<Self::L>,
//     ) -> super::Result<Self, Self::L> {
//         match Self::match_literal() {
//             Some(literal) => {
//                 let follow = meta.follow.get(&Self::type_id());
//                 parser.parse_token_lit(Self::token_type(), literal, follow).map(Self::from)
//             },
//             None => {
//                 parser.parse_token(Self::token_type()).map(Self::from)
//             }
//         }
//     }
// }
