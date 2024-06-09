use teleparse::prelude::*;
#[derive_lexicon]
pub enum MyToken {
    #[teleparse(terminal(Zero = "0", Another = "0"))]
    Integer,
}
fn main() {}
