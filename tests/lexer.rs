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
    ignore(r#"\s"#), // ignore whitespaces, separate multiple with comma
)]
pub enum TokenType {
    /// Numbers in the expression
    #[teleparse(regex(r#"\d+"#), terminal(Integer, Zero = "0"))]
    Integer,
    /// The 4 basic operators
    #[teleparse(terminal(OpAdd = "+", OpSub = "-", OpMul = "*", OpDiv = "/",))]
    Operator,
    /// Parentheses
    #[teleparse(terminal(ParamOpen = "(", ParamClose = ")"))]
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
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((0, 1), TokenType::Integer)))
    );

    let mut lexer = TokenType::lexer("(").unwrap();
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((0, 1), TokenType::Param)))
    );

    let mut lexer = TokenType::lexer("*").unwrap();
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((0, 1), TokenType::Operator)))
    );
}

#[test]
fn basic() {
    let source = "3+4*(5-6)/7";
    let mut lexer = TokenType::lexer(source).unwrap();
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((0, 1), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((1, 2), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((2, 3), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((3, 4), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((4, 5), TokenType::Param)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((5, 6), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((6, 7), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((7, 8), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((8, 9), TokenType::Param)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((9, 10), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((10, 11), TokenType::Integer)))
    );
    assert_eq!(lexer.next(), (None, None));
}

#[test]
fn with_ignore() {
    // -----------0123456789101214
    let source = "3 + 4  *( 5 -6";
    let mut lexer = TokenType::lexer(source).unwrap();
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((0, 1), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((2, 3), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((4, 5), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((7, 8), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((8, 9), TokenType::Param)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((10, 11), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((12, 13), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((13, 14), TokenType::Integer)))
    );
    assert_eq!(lexer.next(), (None, None));
}

#[test]
fn invalid() {
    // -----------01234567891012141618202224262830
    let source = "3+ 4 what is (this 5)   invalid";
    let mut lexer = TokenType::lexer(source).unwrap();
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((0, 1), TokenType::Integer)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((1, 2), TokenType::Operator)))
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((3, 4), TokenType::Integer)))
    );
    // the invalid part is "what is", then starting from the space,
    // it was able to ignore the space and find the valid "("
    assert_eq!(
        lexer.next(),
        (
            Some((5, 12).into()),
            Some(Token::new((13, 14), TokenType::Param))
        )
    );
    assert_eq!(
        lexer.next(),
        (
            Some((14, 18).into()),
            Some(Token::new((19, 20), TokenType::Integer))
        )
    );
    assert_eq!(
        lexer.next(),
        (None, Some(Token::new((20, 21), TokenType::Param)))
    );
    assert_eq!(lexer.next(), (Some((24, 31).into()), None));
}
