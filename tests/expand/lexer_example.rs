use llnparse::prelude::*;

#[llnparse_derive(Lexer)]
#[llnparse(
    token(TokenType),
    ignore = r#"^\s+"#, // ignore whitespaces
    Integer = r#"^\d+"#,
    Operator = r#"^[\+\-\*/]"#,
    Param = r#"^[\(\)]"#,
)]
pub struct Lexer<'s> {
    state: LexerState<'s>
}
