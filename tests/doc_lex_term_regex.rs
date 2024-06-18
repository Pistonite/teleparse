use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(terminal_parse)]
pub enum MyToken {
    #[teleparse(regex(r"\w+"), terminal(Ident, KwClass = "class"))]
    Word,
}

fn main() {
    let source = "class";
    // can be parsed as Ident and KwClass
    assert_eq!(
        Ident::parse(source),
        Ok(Some(Ident::from_span(0..5)))
    );
    assert_eq!(
        KwClass::parse(source),
        Ok(Some(KwClass::from_span(0..5)))
    );
    // other words can only be parsed as Ident
    let source = "javascript";
    assert_eq!(
        Ident::parse(source),
        Ok(Some(Ident::from_span(0..10)))
    );
    assert_eq!(
        KwClass::parse(source),
        Ok(None)
    );
}
