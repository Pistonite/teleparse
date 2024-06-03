use crate::lex::{Token, TokenSrc};
use crate::syntax::Error;
use crate::{Lexicon, Span};

pub struct Info<'s, L: Lexicon> {
    /// The source code to parse
    pub source: &'s str,
    /// The parts of the source code that were unable to be parsed
    pub invalid_source: Vec<Span>,
    /// The tokens that were unable to be parsed with the syntax
    pub invalid_tokens: Vec<Token<L>>,
    /// The valid tokens parsed
    pub tokens: Vec<Token<L>>,
    /// The encountered tokens that were marked with extract
    pub extracted_tokens: Vec<Token<L>>,
    /// Errors encountered during parsing
    pub errors: Vec<Error<L>>,
}

impl<'s, L: Lexicon> Info<'s, L> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            invalid_source: Vec::new(),
            invalid_tokens: Vec::new(),
            tokens: Vec::new(),
            extracted_tokens: Vec::new(),
            errors: Vec::new(),
        }
    }
    /// 
    pub fn to_src(&self, token: &Token<L>) -> TokenSrc<'s, L> {
        token.to_src(self.source)
    }

    pub fn eof(&self) -> Span {
        Span::new(self.source.len(), self.source.len())
    }
}
