//! Implementation of a lexer based on a set of rules
//!
//! See [`lex`](super) module-level documentation for more information.

use regex::Regex;

use super::{Error, Lexicon, Pos, Span, Token};

/// Trait for lexer
///
/// See [module level documentation](super) for more information
pub trait Lexer<'s> {
    type L: Lexicon;

    /// Read the next token from source code
    ///
    /// If a token cannot be produced, one character will be skipped
    /// and the lexer will try again, until one valid token is produced.
    /// The invalid skipped characters will be returned as a span (first of the tuple)
    /// and the token produced will be returned as the second of the tuple.
    fn next(&mut self) -> (Option<Span>, Option<Token<Self::L>>);
}

///////////////////////////////////////////////////////////
// Implementation (Rule-based)
///////////////////////////////////////////////////////////

/// The lexer implementation used when deriving [`Lexer`], based on pattern-matching rules
pub struct LexerImpl<'s, L: Lexicon+'static> {
    source: &'s str,
    idx: Pos,
    rules: &'static [Rule<L>],
}

impl<'s, L: Lexicon> LexerImpl<'s, L> {
    /// Create a new lexer starting in the beginning of the input
    pub fn new(source: &'s str, rules: &'static [Rule<L>]) -> Result<Self, Error> {
        if rules.is_empty() {
            return Err(Error::NoRules);
        }
        for rule in rules {
            match &rule.pat {
                Pattern::RegexError(pat, error) => {
                    return Err(Error::InvalidRule(format!("Invalid regex pattern: `{}`: {}", pat, error)));
                }
                _ => {}
            }
        }
        Ok(Self {
            source,
            idx: 0,
            rules,
        })
    }

    /// Try matching a token from the current position.
    /// Advances self.idx to the position after the token if one is matched
    fn next_internal(&mut self) -> Option<Token<L>> {
        let source_len = self.source.len();
        'outer: while self.idx < source_len {
            let rest = &self.source[self.idx..];
            for rule in self.rules {
                if let Some(len) = rule.pat.get_prefix_len(rest, 0) {
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

impl<'s, L: Lexicon> Lexer<'s> for LexerImpl<'s, L> {
    type L = L;

    fn next(&mut self) -> (Option<Span>, Option<Token<L>>) {
        let source_len = self.source.len();

        let mut has_invalid = false;
        let mut invalid_start = self.idx;
        let mut invalid_end = self.idx;
        let mut token = None;

        while self.idx < source_len {
            if let Some(next) = self.next_internal() {
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
            Some(Span::new(invalid_start, invalid_end))
        } else {
            None
        };

        (invalid_span, token)
    }
}

///////////////////////////////////////////////////////////
// Rule
///////////////////////////////////////////////////////////

/// A rule in a lexer for matching a token or ignoring something
pub struct Rule<L: Lexicon> {
    /// The token type to match. None for ignore
    ty: Option<L>,
    /// The pattern to match, either a regex or a set of literals
    pat: Pattern,
}

impl<L: Lexicon> Rule<L> {
    /// Create a rule for matching a token with a regex
    #[inline]
    pub fn token(ty: L, pat: &str) -> Self {
        Self {
            ty: Some(ty),
            pat: Pattern::new_regex(pat)
        }
    }

    /// Create a rule for matching a token with a set of literals
    #[inline]
    pub fn token_literal(ty: L, pat: &'static[&'static str]) -> Self {
        Self {
            ty: Some(ty),
            pat: Pattern::new_literals(pat)
        }
    }

    /// Create a rule for matching something to ignore with a regex
    #[inline]
    pub fn ignore(pat: &str) -> Self {
        Self {
            ty: None,
            pat: Pattern::new_regex(pat)
        }
    }
}


///////////////////////////////////////////////////////////
// Pattern
///////////////////////////////////////////////////////////

/// A pattern to match in a [`Rule`]
pub enum Pattern {
    /// A regex pattern. 
    ///
    /// Must start with ^ to match the beginning of the string.
    /// This is enforced when using the [`derive_lexicon`](crate::derive_lexicon) macro.
    Regex(Regex),
    /// A set of literals to match
    ///
    /// The second argument is the max length of the literals
    Literals(&'static [&'static str], usize),
    /// An error occurred when creating the regex
    RegexError(String, regex::Error),
}

impl Pattern {
    pub fn new_regex(pat: &str) -> Self {
        match Regex::new(pat) {
            Ok(regex) => Self::Regex(regex),
            Err(e) => Self::RegexError(pat.to_string(), e),
        }
    }

    pub fn new_literals(pat: &'static [&'static str]) -> Self {
        let max_len = pat.iter().map(|s| s.len()).max().unwrap_or(0);
        Self::Literals(pat, max_len)
    }

    /// If this pattern is a prefix of `input` with at least `at_least` bytes, return the length of the prefix
    #[inline]
    pub fn get_prefix_len(&self, input: &str, at_least: usize) -> Option<usize> {
        match self {
            Self::Regex(regex) => {
                regex.find(input).map(|m| m.end())
            }
            Self::Literals(lits, max_len) => {
                if *max_len < at_least {
                    return None;
                }
                for lit in *lits {
                    if input.starts_with(lit) {
                        return Some(lit.len());
                    }
                }
                None
            }
            _ => None,
        }
    }
}
