use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"\s+"#))]
pub enum TokenType {
    #[teleparse(regex(r#"\w+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(OpAdd = "+", OpMul = "*",))]
    Op,
    #[teleparse(terminal(ParenOpen = "(", ParenClose = ")"))]
    Paren,
}

#[derive_syntax]
#[teleparse(root)]
struct E {
    terms: tp::Split<T, OpAdd>,
}

#[derive_syntax]
struct T {
    factors: tp::Split<F, OpMul>,
}
#[derive_syntax]
enum F {
    Ident(Ident),
    Paren((ParenOpen, Box<E>, ParenClose)),
}

#[test]
fn main() -> Result<(), teleparse::GrammarError> {
    let source = "(a+b)*(c+d)";
    let _ = E::parse(source)?;

    Ok(())
}
