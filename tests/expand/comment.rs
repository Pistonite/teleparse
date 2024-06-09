use teleparse::prelude::*;

#[derive_lexicon]
pub enum MyToken {
    #[teleparse(regex(r"/\*([^\*]|(\*[^/]))*\*/"))]
    Comment,
}

fn main() {
    let input = "/* This is a comment */";
    let mut lexer = MyToken::lexer(input).unwrap();
    assert_eq!(lexer.next(), (None, Some(Token::new(0..23, MyToken::Comment))));

    // this will tell the parser to put this token aside 
    assert!(MyToken::Comment.should_extract());
}
