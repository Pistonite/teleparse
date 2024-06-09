//! Test utilities

use crate::{lex::Token, GrammarError, Lexer, Lexicon, Span, derive_lexicon};

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

#[derive_lexicon]
#[teleparse(
    ignore(r#"\s+"#), // ignore whitespaces, separate multiple with comma
)]
pub enum MathTokenType {
    #[teleparse(regex(r#"\w+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(
        OpAdd = "+",
        OpMul = "*",
    ))]
    Op,
    /// Parentheses
    #[teleparse(terminal(
        ParamOpen = "(",
        ParamClose = ")"
    ))]
    Param,
}

pub mod prelude {
    macro_rules! assert_not_ll1 {
        ($pt:ty, $err:expr) => {
            use $crate::{AbstractSyntaxRoot, ParseRoot};
            let err = if let Err(e) = <$pt as ParseTree>::AST::metadata() {
                e.clone()
            } else {
                panic!("Expected {} to be not LL(1), but it is", stringify!($pt));
            };
            assert_eq!(err, $err);
            assert!(<$pt>::parse("").is_err());
        }
    }
    pub(crate) use assert_not_ll1;

}
