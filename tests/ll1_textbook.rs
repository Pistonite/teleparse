use teleparse::prelude::*;

#[teleparse_derive(TokenType)]
#[teleparse(
    ignore(r#"^\s+"#), // ignore whitespaces, separate multiple with comma
)]
pub enum TokenType {
    #[teleparse(regex(r#"^\w+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(
        OpAdd = "+",
        OpMul = "*",
    ))]
    Op,
    /// Parentheses
    #[teleparse(terminal(
        ParamOpen = "(",
        ParamClose = ")"
    ))]
    Param,
}
