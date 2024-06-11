
use crate::parser::ParseTree;
use crate::syntax::{Result as SynResult, Metadata};
use crate::{AbstractSyntaxTree, ToSpan, Parser};

use super::ast_passthrough;

impl<AST: AbstractSyntaxTree> AbstractSyntaxTree for Box<AST> {
    ast_passthrough!(AST);

    fn parse_ast<'s>(
        parser: &mut Parser<'s, Self::L>, 
        meta: &Metadata<Self::L>,
    ) -> SynResult<Self, Self::L> {
        AST::parse_ast(parser, meta).map(|ast| Box::new(ast))
    }
}

impl<AST: ToSpan> ToSpan for Box<AST> {
    fn span(&self) -> crate::Span {
        self.as_ref().span()
    }
}

impl<PT: ParseTree> ParseTree for Box<PT> {
    type AST = Box<PT::AST>;

    fn from_ast<'s>(ast: Self::AST, parser: &mut Parser<'s, <Self::AST as AbstractSyntaxTree>::L>) -> Self {
        Box::new(PT::from_ast(*ast, parser))
    }
}
