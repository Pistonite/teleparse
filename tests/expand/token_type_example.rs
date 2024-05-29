use teleparse::prelude::*;

#[teleparse_derive(TokenType)]
#[teleparse(ignore(r#"^\s+"#))]
pub enum TokenType {
    /// Numbers in the expression
    #[teleparse(regex(r#"^\d+"#), terminal(Integer))]
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
    #[teleparse(terminal(ParamOpen = "(", ParamClose = ")"))]
    Param,
}
