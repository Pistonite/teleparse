//! Implementation of lexer and related utilities
use regex::Regex;

use crate::{Pos, Span, Token, TokenType};

/// Trait for lexer
///
/// ## Note
/// This is normally derived with [`#[teleparse_derive(TokenType)]`](crate::teleparse_derive) on an
/// enum. See [`TokenType`] for more information.
/// 
/// Once derived, you can create a lexer with [`TokenType::lexer()`].
pub trait Lexer<'s> {
    type T: TokenType;

    /// Read the next token from source code
    ///
    /// If a token cannot be produced, one character will be skipped
    /// and the lexer will try again, until one valid token is produced.
    /// The invalid skipped characters will be returned as a span (first of the tuple)
    /// and the token produced will be returned as the second of the tuple.
    fn next(&mut self) -> (Option<Span>, Option<Token<Self::T>>);
}

/// A rule in a lexer for matching a token or ignoring something
///
/// This is usually used internally when deriving [`TokenType`]
pub struct LexerRule<T: TokenType> {
    /// The token type to match. None for ignore
    ty: Option<T>,
    /// The pattern to match, either a regex or a set of literals
    pat: Pattern,
}

impl<T: TokenType> LexerRule<T> {
    /// Create a rule for matching a token with a regex
    pub fn token(ty: T, pat: &str) -> Self {
        Self {
            ty: Some(ty),
            pat: Pattern::Regex(Regex::new(pat).unwrap())
        }
    }

    /// Create a rule for matching a token with a set of literals
    pub fn token_literal(ty: T, pat: &'static[&'static str]) -> Self {
        Self {
            ty: Some(ty),
            pat: Pattern::Literals(pat)
        }
    }

    /// Create a rule for matching something to ignore with a regex
    pub fn ignore(pat: &str) -> Self {
        Self {
            ty: None,
            pat: Pattern::Regex(Regex::new(pat).unwrap())
        }
    }
}

/// State of a lexer while parsing some source code
pub struct LexerState<'s> {
    source: &'s str,
    idx: Pos,
}

impl<'s> LexerState<'s> {
    /// Create a new lexer state starting in the beginning of the source
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            idx: 0,
        }
    }

    /// Implementation for [`Lexer::next`] with a set of rules
    pub fn next<T: TokenType>(&mut self, rules: &[LexerRule<T>]) -> (Option<Span>, Option<Token<T>>) {
        let source_len = self.source.len();

        let mut has_invalid = false;
        let mut invalid_start = self.idx;
        let mut invalid_end = self.idx;
        let mut token = None;

        while self.idx < source_len {
            if let Some(next) = self.next_internal(rules) {
                token = Some(next);
                break;
            }
            // if no invalid detected so far, check if there were
            // ignored tokens, since we don't want those to be considered
            // invalid
            if !has_invalid {
                invalid_start = self.idx;
                if self.idx >= source_len {
                    // we have ignored everything to the end
                    break;
                }
            }
            // no match was found but there's more source
            // skip one character and retry
            self.idx += 1;
            has_invalid = true;
            invalid_end = self.idx;
        }
        let invalid_span = if has_invalid {
            // note this might also include valid tokens that are ignored
            Some(Span::new(invalid_start, invalid_end))
        } else {
            None
        };

        (invalid_span, token)
    }

    /// Try matching a token from the current position.
    /// Advances self.idx to the position after the token if one is matched
    fn next_internal<T: TokenType>(&mut self, rules: &[LexerRule<T>]) -> Option<Token<T>> {
        let source_len = self.source.len();
        'outer: while self.idx < source_len {
            let rest = &self.source[self.idx..];
            for rule in rules {
                if let Some(len) = rule.pat.get_prefix_len(rest) {
                    let start = self.idx;
                    self.idx += len;
                    let ty = match rule.ty {
                        None => continue 'outer,
                        Some(x) => x
                    };
                    return Some(Token::new((start, self.idx), ty));
                }
            }
            // no rule matched
            return None;
        }
        
        None
    }
}

/// A pattern to match in a lexer
pub enum Pattern {
    /// A regex pattern. Must start with ^ to match the beginning of the string.
    /// This is enforced by the derive macro attributes
    Regex(Regex),
    /// A set of literals to match
    Literals(&'static [&'static str]),
}

impl Pattern {
    /// If this pattern is a prefix of the input, return the length of the prefix
    pub fn get_prefix_len(&self, input: &str) -> Option<usize> {
        match self {
            Self::Regex(regex) => {
                regex.find(input).map(|m| m.end())
            }
            Self::Literals(lits) => {
                for lit in *lits {
                    if input.starts_with(lit) {
                        return Some(lit.len());
                    }
                }
                None
            }
        }
    }
}
