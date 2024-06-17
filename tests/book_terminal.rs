use teleparse::prelude::*;
use teleparse::GrammarError;

#[derive_lexicon]
#[teleparse(ignore(r#"\s+"#), terminal_parse)]
pub enum TokenType {
    #[teleparse(regex(r#"\w+"#), terminal(Id))]
    Ident,
}

#[derive_syntax]
#[teleparse(root)]
#[derive(Debug, PartialEq)] // for assert_eq!
struct ThreeIdents(Id, Id, Id);

#[test]
fn main() -> Result<(), GrammarError>{
    let t = ThreeIdents::parse("a b c")?;
    assert_eq!(t, Some(
        ThreeIdents(
            Id::from_span(0..1),
            Id::from_span(2..3),
            Id::from_span(4..5),
        )
    ));

    let pizza = Id::parse("pizza")?;
    assert_eq!(pizza, Some(Id::from_span(0..5)));

    Ok(())
}
