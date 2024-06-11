//! Blanket implementation for std types in SyntaxTree

use std::{collections::HashSet, marker::PhantomData};

use crate::{Lexer, Parser, Span, Start, SyntaxResult, SyntaxResultExt, SyntaxTree, SyntaxTreeParser};

impl<ST: SyntaxTree> SyntaxTree for Box<ST> {
    type T = ST::T;
    type AST = Box<ST::AST>;

}
