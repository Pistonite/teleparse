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
    first: T,
    rest: Eprime,
}

// Eplus has to be a separate struct because of recursion when
// computing trait satisfaction. See chapter 3.2 Note on Recursion for more details
#[derive_syntax]
struct Eprime(tp::Option<Eplus>);

#[derive_syntax]
struct Eplus {
    op: OpAdd,
    _t: T,
    rest: Box<Eprime>,
}

#[derive_syntax]
struct T {
    first: F,
    rest: Tprime,
}

#[derive_syntax]
struct Tprime(tp::Option<Tstar>);

#[derive_syntax]
struct Tstar {
    op: OpMul,
    _f: F,
    rest: Box<Tprime>,
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
