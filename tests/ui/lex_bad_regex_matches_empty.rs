use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(""))]
    MatchEmpty, 
}

fn main() {}
