use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r#"board"#), terminal(Board, Keyboard = "keyboard"))]
    DoesNotMatchTerminal, 
}

fn main() {}
