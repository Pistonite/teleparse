use teleparse::prelude::*;

#[derive_lexicon]
pub enum TokenType {
    #[teleparse(terminal(
        OpAdd = "+", 
        OpSub = "-", 
        OpMul = "*", 
        OpDiv = "/",
    ), regex(r#"^[\+\-\*/]"#))]
    Operator,
}

fn main() {}
