//! Test utilities

use crate::{lex::Token, GrammarError, Lexer, Lexicon, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestTokenType {
    A,
    B,
    C
}

impl Lexicon for TestTokenType {
    type Bit = u8;
    type Lexer<'s> = LexerStub;
    type Map<T: Default+Clone> = [T; 3];

    fn id(&self) -> usize {
        match self {
            TestTokenType::A => 0,
            TestTokenType::B => 1,
            TestTokenType::C => 2,
        }
    }

    fn from_id(id: usize) -> Self {
        match id {
            0 => TestTokenType::A,
            1 => TestTokenType::B,
            _ => TestTokenType::C,
        }
    }

    fn to_bit(&self) -> Self::Bit {
        match self {
            TestTokenType::A => 1,
            TestTokenType::B => 2,
            TestTokenType::C => 4,
        }
    }

    fn first() -> Self {
        TestTokenType::A
    }

    fn next(&self) -> Option<Self> {
        match self {
            TestTokenType::A => Some(TestTokenType::B),
            TestTokenType::B => Some(TestTokenType::C),
            TestTokenType::C => None,
        }
    }

    fn should_extract(&self) -> bool {
        false
    }

    fn lexer<'s>(_: &'s str) -> Result<Self::Lexer<'s>, GrammarError> {
        Ok(LexerStub)
    }
}

pub struct LexerStub;

impl<'s> Lexer<'s> for LexerStub {
    type L = TestTokenType;

    fn next(&mut self) -> (Option<Span>, Option<Token<Self::L>>) {
        (None, None)
    }
}
