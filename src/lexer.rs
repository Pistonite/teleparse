//! Implementation of lexer and related utilities
use regex::Regex;

use crate::{Pos, Span, Token, TokenType};

/// Trait for lexer
///
/// ## Note
/// This is normally derived with [`#[teleparse_derive(TokenType)]`](crate::teleparse_derive) on an
/// enum. See [`TokenType`] for more information.
/// 
pub trait Lexer<'s> {
    type T: TokenType;

    /// Create a new lexer from the source code
    fn new(source: &'s str) -> Self;

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
/// This is usually used internally when deriving [`Lexer`]
pub struct LexerRule<T: TokenType> {
    /// The token type to match. None for ignore
    ty: Option<T>,
    pat: Pattern,
}

impl<T: TokenType> LexerRule<T> {
    /// Create a rule for matching a token
    pub fn token(ty: T, pat: &str) -> Self {
        Self {
            ty: Some(ty),
            pat: Pattern::Regex(Regex::new(pat).unwrap())
        }
    }

    pub fn token_literal(ty: T, pat: &'static[&'static str]) -> Self {
        Self {
            ty: Some(ty),
            pat: Pattern::Literals(pat)
        }
    }

    /// Create a rule for matching something to ignore
    pub fn ignore(pat: &str) -> Self {
        Self {
            ty: None,
            pat: Pattern::Regex(Regex::new(pat).unwrap())
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
                if let Some(len) = rule.pat.find_prefix(rest) {
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

pub enum Pattern {
    Regex(Regex),
    Literals(&'static [&'static str]),
}

// impl From<&'static str> for Pattern {
//     fn from(s: &'static str) -> Self {
//         Self::Regex(Regex::new(s).unwrap())
//     }
// }
//
// impl From<&'static [&'static str]> for Pattern {
//     fn from(s: &'static [&'static str]) -> Self {
//         Self::Literals(s)
//     }
// }

impl Pattern {
    pub fn find_prefix(&self, haystack: &str) -> Option<usize> {
        match self {
            Self::Regex(regex) => {
                regex.find(haystack).map(|m| m.end())
            }
            Self::Literals(lits) => {
                for lit in *lits {
                    if haystack.starts_with(lit) {
                        return Some(lit.len());
                    }
                }
                None
            }
        }
    }
}
