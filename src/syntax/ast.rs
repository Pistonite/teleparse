use std::any::TypeId;
use std::borrow::Cow;
use std::collections::BTreeSet;

use crate::{GrammarError, Lexicon, Parser, ToSpan};

use super::{First, FirstBuilder, Follow, FollowBuilder, Jump, Metadata};


/// An AST node
///
/// See [module-level documentation](super) for more information.
#[doc(alias = "Production")]
pub trait AbstractSyntaxTree: Sized + ToSpan + 'static {
    /// The token type of the AST node
    type L: Lexicon + 'static;

    /// Get the unique type id of the AST node,
    /// which represents one production in the grammar (multiple production in case of a
    /// union/enum)
    #[inline]
    fn type_id() -> TypeId {
        TypeId::of::<Self>()
    }

    /// Get the type name for the AST node for debugging
    #[inline]
    fn debug() -> Cow<'static, str>{
        Cow::Borrowed(std::any::type_name::<Self>())
    }

    /// If this AST is possible to produce epsilon. Used to calculate left recursion before
    /// constructing the FIRST function. This may traverse the tree to check if any child AST node
    /// could produces epsilon.
    fn produces_epsilon() -> bool;

    /// Check if the grammar at this AST node is left recursive
    fn check_left_recursive(stack: &mut Vec<String>, set: &mut BTreeSet<TypeId>) -> Result<(), GrammarError>;
    
    /// Add the rules for this AST node (recursively) to the FIRST function builder.
    ///
    /// Note this may not terminate if the grammar is left-recursive
    fn build_first(builder: &mut FirstBuilder<Self::L>);
    
    /// Check for conflicts in the FIRST set of this AST node
    fn check_first_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>) -> Result<(), GrammarError>;

    /// Add the rules for this AST node (recursively) to the FOLLOW function builder.
    fn build_follow(builder: &mut FollowBuilder<Self::L>);

    /// Check for conflicts in the FIRST and FOLLOW set of this AST node recursively
    fn check_first_follow_conflict(seen: &mut BTreeSet<TypeId>, first: &First<Self::L>, follow: &Follow<Self::L>) -> Result<(), GrammarError>;

    /// Recursively build the parsing table for this AST node.
    ///
    /// See [Predictive parsing table](`super::jump`) for more information.
    fn build_jump(seen: &mut BTreeSet<TypeId>, parsing: &mut Jump<Self::L>);

    /// Parse this AST node from the input stream
    fn parse<'s>(
        parser: &mut Parser<'s, Self::L>, 
        meta: &Metadata<Self::L>,
    ) -> super::Result<Self, Self::L>;
}
