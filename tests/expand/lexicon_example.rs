use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"^\s+"#))] // ignore whitespaces
pub enum TokenType {
    /// Numbers in the expression
    #[teleparse(regex(r#"^\d+"#), terminal(Integer))]
    Integer,
    /// The 4 basic operators
    #[teleparse(terminal(
        OpAdd = "+", 
        OpMul = "*", 
    ))]
    Operator,
    /// Parentheses
    #[teleparse(terminal(ParamOpen = "(", ParamClose = ")"))]
    Param,
}
