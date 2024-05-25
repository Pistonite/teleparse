use llnparse::llnparse_derive;

#[llnparse_derive(TokenType)]
pub enum MyTokenType {
   // extract means the tokens are not relevant in the AST
   // but still collected so you may use them later
   #[llnparse(extract)] 
   Comment,
   Keyword,
   Ident,
}
