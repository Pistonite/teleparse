
use crate::{SyntaxTree, ToSpan};

use super::ast_passthrough;
impl<ST: SyntaxTree> SyntaxTree for Box<ST> {
    type T = ST::T;
    type AST = Box<ST::AST>;
    ast_passthrough!();

    fn try_parse_ast<'s>(
        parser: &mut crate::Parser<'s, Self::T>, 
        meta: &crate::root::RootMetadata<Self::T>,
    ) -> crate::AstResult<Self::T, Self::AST> {
        ST::try_parse_ast(parser, meta).map(|ast| Box::new(ast))
    }
}

impl<ST: ToSpan> ToSpan for Box<ST> {
    fn span(&self) -> crate::Span {
        self.as_ref().span()
    }
}
