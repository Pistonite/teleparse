use teleparse::prelude::*;

// This tests the "textbook" grammar:
// E -> T E'
// E' -> + T E' | ε
// T -> F T'
// T' -> * F T' | ε
// F -> ( E ) | id

#[derive_lexicon]
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

// pub enum Factor {
//     Paren((ParamOpen, Box<Expr>, ParamClose)),
//     Ident
// }
