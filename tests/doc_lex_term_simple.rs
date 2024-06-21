use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(terminal_parse)]
pub enum MyToken {
    #[teleparse(regex(r"\w+"), terminal(Ident))]
    Ident,
}

fn main() {
    assert_eq!(Ident::parse("hell0"), Ok(Some(Ident::from_span(0..5))));
}
