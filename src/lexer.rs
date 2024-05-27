use regex::Regex;

use crate::{Pos, Span, Token, TokenType};

/// Trait for lexer
///
/// ## Note
/// This is normally derived with [`#[llnparse_derive(Lexer)]`](crate::llnparse_derive) on a struct
/// with a single named field of type [`LexerState`]. Then use the `llnparse` attribute to declare
/// the token type and rules.
///
/// ## Example
/// ```rust
#[doc = include_str!("../tests/expand/lexer_example.rs")]
/// ```
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
    pub ty: Option<T>,
    pub regex: Regex
}

impl<T: TokenType> LexerRule<T> {
    /// Create a rule for matching a token
    ///
    /// Returns None if the regex is invalid
    pub fn token(ty: T, regex: &str) -> Option<Self> {
        Some(Self {
            ty: Some(ty),
            regex: Regex::new(regex).ok()?
        })
    }

    /// Create a rule for matching something to ignore
    ///
    /// Returns None if the regex is invalid
    pub fn ignore(regex: &str) -> Option<Self> {
        Some(Self {
            ty: None,
            regex: Regex::new(regex).ok()?
        })
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
                if let Some(mat) = rule.regex.find(rest) {
                    let len = mat.end();
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
