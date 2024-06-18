use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r"\"))]
pub enum MyToken {
    Invalid, 
}

fn main() {}
