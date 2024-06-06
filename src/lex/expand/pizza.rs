use teleparse::prelude::*;

#[derive_lexicon]
#[teleparse(terminal_parse)]
pub enum MyToken {
    #[teleparse(terminal(Pizza = "pizza", Pasta = "pasta"))]
    Food,
}
