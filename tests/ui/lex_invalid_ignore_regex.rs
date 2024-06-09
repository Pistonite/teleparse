use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(ignore(r#"\"#))]
pub enum MyToken {
    #[teleparse(regex(r#"abc"#))]
    Invalid, 
}

fn main() {}
