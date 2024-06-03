
use std::marker::PhantomData;

use crate::{syntax::Metadata, AbstractSyntaxRoot, AbstractSyntaxTree, GrammarError, Lexicon, ToSpan};

use super::ParseRootIter;

type Parser<'s, AST> = super::Parser<'s, <AST as AbstractSyntaxTree>::L>;

pub trait ParseTree: Sized + ToSpan {
    type AST: AbstractSyntaxTree;

    /// Transform the parsed AST node into the final tree node
    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, Self::AST>) -> Self;
}

pub trait ParseRoot: ParseTree 
where Self::AST : AbstractSyntaxRoot
{
    fn parse(source: &str) -> Result<Option<Self>, GrammarError> {
        super::Parser::new(source)?.parse_one()
    }

    fn iter(source: &str) -> Result<ParseRootIter<'_, <Self::AST as AbstractSyntaxTree>::L, Self>, GrammarError> {
        ParseRootIter::new(super::Parser::new(source)?)
    }

    fn parse_all(source: &str) -> Result<Vec<Self>, GrammarError> {
        super::Parser::new(source)?.parse_all()
    }
}


