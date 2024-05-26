use crate::{Span, Token, TokenType};

pub trait Lexer<'s> {
    type TokenTy: TokenType;

    /// Create a new lexer from the source code
    ///
    /// The source code is not copied, so it must outlive the lexer.
    /// Creating the lexer does not perform parsing. You need to pass
    /// it to a [`SyntaxTree`] to parse the source code.
    fn new(source: &'s str) -> Self;

    /// Read the next token from source code
    ///
    /// If a token cannot be produced, one character will be skipped
    /// and the lexer will try again, until one valid token is produced.
    /// The invalid skipped characters will be returned as a span (first of the tuple)
    /// and the token produced will be returned as the second of the tuple.
    fn next(&mut self) -> (Option<Span>, Option<Token<Self::TokenTy>>);
}
