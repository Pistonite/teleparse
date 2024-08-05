use teleparse::prelude::*;
use teleparse::Parser;

#[derive_lexicon]
#[teleparse(ignore(r#"\s+"#))]
pub enum MathTokenType {
    #[teleparse(regex(r#"[a-zA-Z]+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(OpAdd = "+", OpMul = "*", OpEq = "="))]
    Op,
    #[teleparse(terminal(ParenOpen = "(", ParenClose = ")"))]
    Paren,
    Variable
}

#[derive_syntax]
#[teleparse(root)]
pub struct Assignment {
    #[teleparse(semantic(Variable))]
    pub variable: Ident,
    pub op: OpEq,
    pub expression: Expr,
}

#[derive_syntax]
pub struct Expr {
    terms: tp::Split<Term, OpAdd>,
}

#[derive_syntax]
pub struct Term {
    factors: tp::Split<Factor, OpMul>,
}

#[derive_syntax]
pub enum Factor {
    Ident(Ident),
    Paren((ParenOpen, Box<Expr>, ParenClose)),
}

#[test]
fn test_simple() -> Result<(), teleparse::GrammarError> {
    let source = "a = b + c * d";
    let mut parser = Parser::<MathTokenType>::new(source)?;
    let assignment = parser.parse::<Assignment>()?.unwrap();

    let token = parser.info().tokens.at_span(assignment.variable.span()).unwrap();
    assert!(token.semantic.contains(MathTokenType::Variable));

    let token = parser.info().tokens.at_span(assignment.expression.terms[0].span()).unwrap();
    assert!(!token.semantic.contains(MathTokenType::Variable));

    Ok(())
}
