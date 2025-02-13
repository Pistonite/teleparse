use std::fmt::Debug;

use super::{Lexicon, Span, ToSpan};

///////////////////////////////////////////////////////////
// Token
///////////////////////////////////////////////////////////

/// Item produced by a lexer, which holds the token type and the source span
#[derive(Clone, Copy, PartialEq, Eq, Hash, ToSpan)]
pub struct Token<L: Lexicon> {
    /// Position of this token in the source
    pub span: Span,
    /// Type of this token
    pub ty: L,
}

impl<L: Lexicon> Debug for Token<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "token {:?}({:?})", self.ty, self.span)
    }
}

impl<L: Lexicon> Token<L> {
    /// Create a new Token
    pub fn new<TSpan: Into<Span>>(span: TSpan, ty: L) -> Self {
        Self {
            span: span.into(),
            ty,
        }
    }

    /// Get the source of this token from the entire source input
    #[inline]
    pub fn src<'s>(&self, input: &'s str) -> &'s str {
        self.span.get(input)
    }

    /// Get the content of this token as a [`TokenSrc`] from the entire source input
    #[inline]
    pub fn to_src<'s>(&self, input: &'s str) -> TokenSrc<'s, L> {
        TokenSrc {
            ty: self.ty,
            src: self.src(input),
        }
    }
}

///////////////////////////////////////////////////////////
// TokenSrc
///////////////////////////////////////////////////////////

/// Like [`Token`], but holds a reference to the source spanned by this token instead of the span
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenSrc<'s, L: Lexicon> {
    pub ty: L,
    pub src: &'s str,
}

impl<'s, L: Lexicon> TokenSrc<'s, L> {
    #[inline]
    pub fn new(ty: L, src: &'s str) -> Self {
        Self { ty, src }
    }
}

impl<'s, L: Lexicon> From<(L, &'s str)> for TokenSrc<'s, L> {
    #[inline]
    fn from((ty, src): (L, &'s str)) -> Self {
        Self { ty, src }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::TestTokenType as T;

    #[test]
    fn test_src() {
        let token = Token::new((0, 1), T::A);
        assert_eq!(token.src("abc"), "a");
    }

    #[test]
    fn test_src_empty() {
        let token = Token::new((0, 0), T::A);
        assert_eq!(token.src("abc"), "");
        let token = Token::new((1, 1), T::A);
        assert_eq!(token.src("abc"), "");
        let token = Token::new((1, 0), T::A);
        assert_eq!(token.src("abc"), "");
    }

    #[test]
    fn test_overflows() {
        let token = Token::new(0..100, T::A);
        assert_eq!(token.src("abc"), "abc");
    }
}
