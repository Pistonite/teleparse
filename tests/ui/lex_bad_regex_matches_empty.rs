use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r#"^"#))]
    MatchEmpty, 
}

fn main() {}
