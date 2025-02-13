use teleparse::prelude::*;
#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex("key"), terminal(Key, Keyboard = "keyboard"))]
    DoesNotMatchTerminal, 
}

fn main() {}
