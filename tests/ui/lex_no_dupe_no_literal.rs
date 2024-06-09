use teleparse::prelude::*;
#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r"\d+"), terminal(Integer, FancyInteger))]
    Integer,
}
fn main() {}
