//! # math
//!
//! Example for parsing simple math expressions with integers, + = * / and ()
//! it doesn't include unary operators, and doesn't allow implicit multiplication
//! like (1-2)(3+4) or 2(3+4)
//!

use llnparse::llnparse_derive;

// note that this example is not the only way to define the parser

/// Token types for the lexer
#[llnparse_derive(TokenType)]
pub enum TokenType {
    /// Numbers in the expression (for simplicity, only integers)
    Integer,
    /// The 4 basic operators
    Operator,
    /// Parentheses
    Param,
}

/// Convenience type for the token
type Token = llnparse::Token<TokenType>;

// /// The lexer
// #[llnparse_derive(Lexer)]
// #[lexer(
//     TokenType,
//     ignore(regex(r#"\s+"#)), // ignore whitespaces
//     regex(Integer, r#"\d+"#),
//     regex(Operator, r#"[\+-\*/]"#),
//     // you can also use regex for these, here it's just showing how to use literal
//     literal(Param, "("), 
//     literal(Param, ")")
// )]
// pub struct Lexer;
//
// // The "grammar"
// // This is simplified to allow lists, thus with quotes
// // It reflects how the attributes are actually used later
// // Expr => ExprWithParam | ExprWithoutParam | Integer
// // ExprWithParam => "(" Expr ")"
// // ExprWithoutParam => SepListNoTrail<Term, OpAddSub>
// // Term => SepListNoTrail<Expr, OpMulDiv>
// // OpAddSub => "+" | "-"
// // OpMulDiv => "*" | "/"
//
// /// Convenience type for the syntax tree node
// type Node = llnparse::Node<TokenType>;
//
// /// An expression
// #[derive(llnparse::SyntaxTree)]
// #[token(TokenType)]
// pub enum Expr {
//     /// An expression surrounded by parentheses
//     WithParam(ExprWithParam),
//     /// An expression not surrounded by parentheses
//     #[list(non_empty)] 
//     WithoutParam(llnparse::SepListNoTrail<Term, OpAddSub>),
//     // ^ note this is in order so WithoutParam must be before Integer
//     /// An integer
//     #[token(Integer)]
//     Integer(Token), 
//     // ^ note you can put token here instead of making a separate struct
//     // because Token implements SyntaxTree
// }
//
// /// An expression surrounded by parentheses
// #[derive(llnparse::SyntaxTree)]
// #[token(TokenType)]
// pub struct ExprWithParam {
//     pub node: Node,
//     #[token(Param, "(")]
//     pub open: Token,
//     pub expr: Box<Expr>, // boxed to avoid infinite size
//     #[token(Param, ")")]
//     pub close: Token,
// }
//
// /// A term with just multiplication and division
// #[derive(llnparse::SyntaxTree)]
// #[token(TokenType)]
// pub struct Term {
//     pub node: Node,
//     #[list(non_empty)]
//     pub operands: llnparse::SepListNoTrail<Expr, OpMulDiv>,
// }
//
// /// An addition or subtraction operator
// #[derive(llnparse::SyntaxTree)]
// #[token(TokenType)]
// pub enum OpAddSub {
//     #[token(Operator, "+")]
//     Add(Token),
//     #[token(Operator, "-")]
//     Sub(Token),
// }
//
// /// An multiplication or division operator
// #[derive(llnparse::SyntaxTree)]
// #[token(TokenType)]
// pub enum OpMulDiv {
//     #[token(Operator, "*")]
//     Mul(Token),
//     #[token(Operator, "/")]
//     Div(Token),
// }

fn main() {
}
