use llnparse::prelude::*;
pub struct Lexer<'s> {
    state: LexerState<'s>,
}
#[automatically_derived]
const _: () = {
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct RULES {
        __private_field: (),
    }
    #[doc(hidden)]
    static RULES: RULES = RULES { __private_field: () };
    impl ::lazy_static::__Deref for RULES {
        type Target = [llnparse::LexerRule<TokenType>; 4usize];
        fn deref(&self) -> &[llnparse::LexerRule<TokenType>; 4usize] {
            #[inline(always)]
            fn __static_ref_initialize() -> [llnparse::LexerRule<TokenType>; 4usize] {
                {
                    [
                        llnparse::LexerRule::ignore(
                            llnparse::dep::Regex::new(r#"^\s+"#).unwrap(),
                        ),
                        llnparse::LexerRule::token(
                            TokenType::Integer,
                            llnparse::dep::Regex::new(r#"^\d+"#).unwrap(),
                        ),
                        llnparse::LexerRule::token(
                            TokenType::Operator,
                            llnparse::dep::Regex::new(r#"^[\+\-\*/]"#).unwrap(),
                        ),
                        llnparse::LexerRule::token(
                            TokenType::Param,
                            llnparse::dep::Regex::new(r#"^[\(\)]"#).unwrap(),
                        ),
                    ]
                }
            }
            #[inline(always)]
            fn __stability() -> &'static [llnparse::LexerRule<TokenType>; 4usize] {
                static LAZY: ::lazy_static::lazy::Lazy<
                    [llnparse::LexerRule<TokenType>; 4usize],
                > = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for RULES {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
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
            use std::ops::Deref;
            self.state.next(RULES.deref())
        }
    }
};
