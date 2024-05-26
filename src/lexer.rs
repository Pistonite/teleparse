use regex::Regex;

use crate::{Pos, Span, Token, TokenType};

pub trait Lexer<'s> {
    type T: TokenType;

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
    fn next(&mut self) -> (Option<Span>, Option<Token<Self::T>>);
}

/// One match rule in a lexer
pub struct LexerRule<T: TokenType> {
    /// The token type to match. None for ignore
    pub ty: Option<T>,
    pub regex: Regex
}

impl<T: TokenType> LexerRule<T> {
    /// Create a rule for matching a token
    pub fn token(ty: T, regex: Regex) -> Self {
        Self {
            ty: Some(ty),
            regex
        }
    }

    /// Create a rule for matching something to ignore
    pub fn ignore(regex: Regex) -> Self {
        Self {
            ty: None,
            regex
        }
    }
}

pub struct LexerState<'s> {
    source: &'s str,
    idx: Pos,
}

impl<'s> LexerState<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            idx: 0,
        }
    }

    pub fn next<T: TokenType>(&mut self, rules: &[LexerRule<T>]) -> (Option<Span>, Option<Token<T>>) {
        let original_start = self.idx;
        let source_len = self.source.len();

        let mut has_invalid = false;
        let mut token = None;

        while self.idx < source_len {
            if let Some(next) = self.next_internal(rules) {
                token = Some(next);
                break;
            }
            // skip one character and retry
            self.idx += 1;
            has_invalid = true;

        }
        let invalid_span = if has_invalid {
            // note this might also include valid tokens that are ignored
            Some(Span::new(original_start, self.idx))
        } else {
            None
        };

        (invalid_span, token)
    }

    /// Try matching a token from the current position.
    /// Advances self.idx to the position after the token if one is matched
    fn next_internal<T: TokenType>(&mut self, rules: &[LexerRule<T>]) -> Option<Token<T>> {
        let source_len = self.source.len();
        while self.idx < source_len {
            let rest = &self.source[self.idx..];
            for rule in rules {
                if let Some(mat) = rule.regex.find(rest) {
                    let len = mat.end();
                    self.idx += len;
                    let ty = match rule.ty {
                        None => continue,
                        Some(x) => x
                    };
                    return Some(Token::new((self.idx, self.idx+len), ty));
                }
            }
            // no rule matched
            return None;
        }

        None
    }
}
