use itertools::Itertools;
use teleparse_macros::ToSpan;

use crate::{Lexicon, Span};

use super::FirstSet;

/// Error encountered during syntax analysis
#[derive(Debug, Clone, ToSpan, PartialEq)]
pub struct Error<L: Lexicon> {
    pub span: Span,
    pub data: ErrorKind<L>,
}

impl<L: Lexicon> Error<L> {
    pub fn new<S: Into<Span>>(span: S, data: ErrorKind<L>) -> Self {
        Self { span: span.into(), data }
    }

    pub fn message(&self, input: &str) -> String {
        match &self.data {
            ErrorKind::Custom(msg) => msg.clone(),
            ErrorKind::UnexpectedCharacters => {
                format!("Unexpected: {}", self.span.get(input))
            },
            ErrorKind::UnexpectedTokens => {
                format!("Unexpected token(s): {}", self.span.get(input))
            },
            ErrorKind::Expecting(set) => {
                let set = set.as_terminal_set().to_repr().into_iter().join(", ");
                format!("Expecting one of {}", set)
            },
            ErrorKind::UnexpectedEof => "Unexpected end of file".to_string(),
            ErrorKind::UnexpectedNoAdvanceInLoop => "Unexpected: Parser did not advance in a loop. The grammar is probably not LL(1), and this is a bug since the parser should catch that before parsing.".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind<L: Lexicon> {
    Custom(String),
    UnexpectedCharacters,
    UnexpectedTokens,
    Expecting(FirstSet<L>),
    UnexpectedEof,
    UnexpectedNoAdvanceInLoop
}

/// Result of parsing an AST node
pub enum Result<T, L: Lexicon> {
    /// The AST node was successfully parsed, with the corresponding tokens consumed
    Success(T),
    /// The parser panicked while parsing the AST node, but it was able to skip some tokens and
    /// recover.
    Recovered(T, Vec<Error<L>>),
    /// The parser panicked while parsing the AST node, and was unable to recover.
    /// The parser might have advanced its position in the input.
    Panic(Vec<Error<L>>),
}

impl<T, L: Lexicon> Result<T, L> {
    #[inline]
    pub fn map<T2, F: FnOnce(T) -> T2>(self, f: F) -> Result<T2, L> {
        match self {
            Self::Success(obj) => Result::Success(f(obj)),
            Self::Recovered(obj, errors) => Result::Recovered(f(obj), errors),
            Self::Panic(errors) => Result::Panic(errors),
        }
    }
    
}
