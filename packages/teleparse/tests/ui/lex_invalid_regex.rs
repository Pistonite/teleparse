use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r"\"))]
    Invalid, 
}

fn main() {}
