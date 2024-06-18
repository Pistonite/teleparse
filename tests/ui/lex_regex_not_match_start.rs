use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex("board"), terminal(Board, Keyboard = "keyboard"))]
    DoesNotMatchTerminal, 
}

fn main() {}
