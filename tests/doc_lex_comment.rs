use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r"/\*([^\*]|(\*[^/]))*\*/"))]
    Comment,
}

fn main() {
    let input = "/* This is a comment */";
    // you can call `lexer` to use a standalone lexer without a Parser
    let mut lexer = MyToken::lexer(input).unwrap();
    // the lexer will not ignore comments
    assert_eq!(lexer.next(), (None, Some(Token::new(0..23, MyToken::Comment))));
    // `should_extract` will tell the lexer to not return the token to the Parser
    assert!(MyToken::Comment.should_extract());
}
