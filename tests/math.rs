//! # math
//!
//! Example for parsing simple math expressions with integers, + = * / and ()
//! it doesn't include unary operators, and doesn't allow implicit multiplication
//! like (1-2)(3+4) or 2(3+4)
//!

use teleparse::prelude::*;

// note that this example is not the only way to define the parser

/// Token types for the lexer
#[derive_lexicon]
#[teleparse(
    ignore(r#"^\s+"#), // ignore whitespaces, separate multiple with comma
)]
pub enum TokenType {
    /// Numbers in the expression
    #[teleparse(regex(r#"^\d+"#), terminal(Integer, Zero = "0"))]
    Integer,
    /// The 4 basic operators
    #[teleparse(terminal(
        OpAdd = "+",
        OpSub = "-",
        OpMul = "*",
        OpDiv = "/",
    ))]
    Operator,
    /// Parentheses
    #[teleparse(terminal(
        ParamOpen = "(",
        ParamClose = ")"
    ))]
    Param,
}

#[test]
fn empty() {
    let mut lexer = TokenType::lexer("").unwrap();
    assert_eq!(lexer.next(), (None, None));

    let mut lexer = TokenType::lexer("   ").unwrap();
    assert_eq!(lexer.next(), (None, None));
}

#[test]
fn single() {
    let mut lexer = TokenType::lexer("3").unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new((0,1), TokenType::Integer))));

    let mut lexer = TokenType::lexer("(").unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new((0,1), TokenType::Param))));

    let mut lexer = TokenType::lexer("*").unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new((0,1), TokenType::Operator))));

}

#[test]
fn basic() {
    let source = "3+4*(5-6)/7";
    let mut lexer = TokenType::lexer(source).unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new((0,1), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((1,2), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((2,3), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((3,4), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((4,5), TokenType::Param))));
    assert_eq!(lexer.next(), (None, Some(Token::new((5,6), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((6,7), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((7,8), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((8,9), TokenType::Param))));
    assert_eq!(lexer.next(), (None, Some(Token::new((9,10), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((10,11), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, None));
}

#[test]
fn with_ignore() {
    // -----------0123456789101214
    let source = "3 + 4  *( 5 -6";
    let mut lexer = TokenType::lexer(source).unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new((0,1), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((2,3), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((4,5), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((7,8), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((8,9), TokenType::Param))));
    assert_eq!(lexer.next(), (None, Some(Token::new((10,11), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((12,13), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((13,14), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, None));
}

#[test]
fn invalid() {
    // -----------01234567891012141618202224262830
    let source = "3+ 4 what is (this 5)   invalid";
    let mut lexer = TokenType::lexer(source).unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new((0,1), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((1,2), TokenType::Operator))));
    assert_eq!(lexer.next(), (None, Some(Token::new((3,4), TokenType::Integer))));
    // the invalid part is "what is", then starting from the space,
    // it was able to ignore the space and find the valid "("
    assert_eq!(lexer.next(), (Some((5,12).into()), Some(Token::new((13,14), TokenType::Param))));
    assert_eq!(lexer.next(), (Some((14,18).into()), Some(Token::new((19,20), TokenType::Integer))));
    assert_eq!(lexer.next(), (None, Some(Token::new((20,21), TokenType::Param))));
    assert_eq!(lexer.next(), (Some((24, 31).into()), None));
}
// //
// // // The "grammar"
// // // This is simplified to allow lists, thus with quotes
// // // It reflects how the attributes are actually used later
// // // Expr => ExprWithParam | ExprWithoutParam | Integer
// // // ExprWithParam => "(" Expr ")"
// // // ExprWithoutParam => SepListNoTrail<Term, OpAddSub>
// // // Term => SepListNoTrail<Expr, OpMulDiv>
// // // OpAddSub => "+" | "-"
// // // OpMulDiv => "*" | "/"
// //
// // /// Convenience type for the syntax tree node
// // type Node = llnparse::Node<TokenType>;
// //
// // /// An expression
// // #[derive(llnparse::SyntaxTree)]
// // #[token(TokenType)]
// // pub enum Expr {
// //     /// An expression surrounded by parentheses
// //     WithParam(ExprWithParam),
// //     /// An expression not surrounded by parentheses
// //     WithoutParam(SepList<Term, OpAddSub>),
// //     // ^ note this is in order so WithoutParam must be before Integer
// //     /// An integer
// //     #[token(Integer)]
// //     Integer(Token), 
// //     // ^ note you can put token here instead of making a separate struct
// //     // because Token implements SyntaxTree
// // }
// //
// // /// An expression surrounded by parentheses
// // #[derive(llnparse::SyntaxTree)]
// // #[token(TokenType)]
// // pub struct ExprWithParam {
// //     pub node: Node,
// //     #[token(Param, "(")]
// //     pub open: Token,
// //     pub expr: Box<Expr>, // boxed to avoid infinite size
// //     #[token(Param, ")")]
// //     pub close: Token,
// // }
// //
// // /// A term with just multiplication and division
// // #[derive(llnparse::SyntaxTree)]
// // #[token(TokenType)]
// // pub struct Term {
// //     pub node: Node,
// //     #[list(non_empty)]
// //     pub operands: llnparse::SepListNoTrail<Expr, OpMulDiv>,
// // }
// //
// // /// An addition or subtraction operator
// // #[derive(llnparse::SyntaxTree)]
// // #[token(TokenType)]
// // pub enum OpAddSub {
// //     #[token(Operator, "+")]
// //     Add(Token),
// //     #[token(Operator, "-")]
// //     Sub(Token),
// // }
// //
// // /// An multiplication or division operator
// // #[derive(llnparse::SyntaxTree)]
// // #[token(TokenType)]
// // pub enum OpMulDiv {
// //     #[token(Operator, "*")]
// //     Mul(Token),
// //     #[token(Operator, "/")]
// //     Div(Token),
// // }
//
// fn main() {
// }
