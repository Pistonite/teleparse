use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    // fail! of course it has to match the actual literal
    #[teleparse(regex("foo"), terminal(Key, Bar = "bar"))]
    DoesNotMatchTerminal, 
}

fn main() {}
