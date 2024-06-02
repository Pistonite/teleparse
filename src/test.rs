//! Test utilities

use crate::table::LitSet;
use crate::{Lexer, Span, Token, TokenType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestTokenType {
    A,
    B,
    C
}

impl TokenType for TestTokenType {
    type Bit = u8;
    type Lexer<'s> = LexerStub;
    type Set = [LitSet; 3];
    type Ctx = ();

    fn id(&self) -> usize {
        match self {
            TestTokenType::A => 0,
            TestTokenType::B => 1,
            TestTokenType::C => 2,
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

    fn lexer<'s>(_: &'s str) -> Self::Lexer<'s> {
        LexerStub
    }
}

pub struct LexerStub;

impl<'s> Lexer<'s> for LexerStub {
    type T = TestTokenType;

    fn next(&mut self) -> (Option<Span>, Option<Token<Self::T>>) {
        (None, None)
    }
}
