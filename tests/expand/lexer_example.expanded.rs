use llnparse::prelude::*;
#[repr(u8)]
pub enum TokenType {
    Integer = 0x1u8,
    Operator = 0x2u8,
    Param = 0x4u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for TokenType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TokenType::Integer => "Integer",
                TokenType::Operator => "Operator",
                TokenType::Param => "Param",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TokenType {
    #[inline]
    fn clone(&self) -> TokenType {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for TokenType {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TokenType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TokenType {
    #[inline]
    fn eq(&self, other: &TokenType) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TokenType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for TokenType {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl llnparse::TokenType for TokenType {
    type Repr = u8;
    #[inline]
    fn should_extract(&self) -> bool {
        match self {
            _ => false,
        }
    }
    #[inline]
    fn to_repr(&self) -> Self::Repr {
        *self as Self::Repr
    }
    #[inline]
    fn first() -> Self {
        Self::Integer
    }
    fn next(&self) -> Option<Self> {
        match self {
            Self::Param => None,
            _ => {
                let repr = self.to_repr();
                let next = repr << 1;
                Some(unsafe { std::mem::transmute(next) })
            }
        }
    }
}
pub struct Lexer<'s> {
    state: LexerState<'s>,
}
#[automatically_derived]
const _: () = {
    #[doc(hidden)]
    fn _the_rules() -> &'static [llnparse::LexerRule<TokenType>; 4usize] {
        static RULES: std::sync::OnceLock<[llnparse::LexerRule<TokenType>; 4usize]> = std::sync::OnceLock::new();
        RULES
            .get_or_init(|| {
                [
                    llnparse::LexerRule::ignore(r#"^\s+"#).unwrap(),
                    llnparse::LexerRule::token(TokenType::Integer, r#"^\d+"#).unwrap(),
                    llnparse::LexerRule::token(TokenType::Operator, r#"^[\+\-\*/]"#)
                        .unwrap(),
                    llnparse::LexerRule::token(TokenType::Param, r#"^[\(\)]"#).unwrap(),
                ]
            })
    }
    impl<'s> llnparse::Lexer<'s> for Lexer<'s> {
        type T = TokenType;
        fn new(source: &'s str) -> Self {
            Self {
                state: llnparse::LexerState::new(source),
            }
        }
        fn next(
            &mut self,
        ) -> (Option<llnparse::Span>, Option<llnparse::Token<Self::T>>) {
            self.state.next(_the_rules())
        }
    }
};
