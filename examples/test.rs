use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"\s")]
pub enum TokenType {
    /// Numbers in the expression
    #[regex(r#"\d+"#)]
    Integer,

    /// The 4 basic operators
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]
    Operator,

    /// Parentheses
    #[token("(")]
    #[token(")")]
    Param,
}

fn main() {
    let source = "  ";

    let mut lex = TokenType::lexer(source);

    println!("{:?}", lex.span());
    println!("{:?}", lex.next());
    println!("{:?}", lex.span());

    println!("{:?}", lex.next());
    println!("{:?}", lex.span());

    println!("{:?}", lex.next());
    println!("{:?}", lex.span());

    println!("{:?}", lex.next());
    println!("{:?}", lex.span());

    println!("{:?}", lex.next());
    println!("{:?}", lex.span());

}
