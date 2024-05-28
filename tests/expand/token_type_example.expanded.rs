use teleparse::prelude::*;
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
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(Integer)`
#[automatically_derived]
pub struct Integer(pub teleparse::Token<TokenType>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for Integer {
        type T = TokenType;
        type Ctx = ();
        type AST = Token<TokenType>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token(TokenType::Integer, parser)
        }
        #[inline]
        fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpAdd = "+")`
#[automatically_derived]
pub struct OpAdd(pub teleparse::Token<TokenType>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for OpAdd {
        type T = TokenType;
        type Ctx = ();
        type AST = Token<TokenType>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenType::Operator, "+", parser)
        }
        #[inline]
        fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpSub = "-")`
#[automatically_derived]
pub struct OpSub(pub teleparse::Token<TokenType>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for OpSub {
        type T = TokenType;
        type Ctx = ();
        type AST = Token<TokenType>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenType::Operator, "-", parser)
        }
        #[inline]
        fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpMul = "*")`
#[automatically_derived]
pub struct OpMul(pub teleparse::Token<TokenType>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for OpMul {
        type T = TokenType;
        type Ctx = ();
        type AST = Token<TokenType>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenType::Operator, "*", parser)
        }
        #[inline]
        fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpDiv = "/")`
#[automatically_derived]
pub struct OpDiv(pub teleparse::Token<TokenType>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for OpDiv {
        type T = TokenType;
        type Ctx = ();
        type AST = Token<TokenType>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenType::Operator, "/", parser)
        }
        #[inline]
        fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(ParamOpen = "(")`
#[automatically_derived]
pub struct ParamOpen(pub teleparse::Token<TokenType>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for ParamOpen {
        type T = TokenType;
        type Ctx = ();
        type AST = Token<TokenType>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenType::Param, "(", parser)
        }
        #[inline]
        fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(ParamClose = ")")`
#[automatically_derived]
pub struct ParamClose(pub teleparse::Token<TokenType>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for ParamClose {
        type T = TokenType;
        type Ctx = ();
        type AST = Token<TokenType>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenType::Param, ")", parser)
        }
        #[inline]
        fn into_parse_tree<'s, L: Lexer<'s, T = Self::T>>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> Self {
            Self(ast)
        }
    }
};
#[automatically_derived]
const _: () = {
    use teleparse::Lexer as _;
    impl teleparse::TokenType for TokenType {
        type Repr = u8;
        type Lexer<'s> = DerivedLexer<'s>;
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
        fn lexer<'s>(source: &'s str) -> Self::Lexer<'s> {
            DerivedLexer::new(source)
        }
    }
    #[doc(hidden)]
    type Rules = [teleparse::lexer::LexerRule<TokenType>; 4usize];
    #[doc(hidden)]
    fn derived_lexer_rules() -> &'static Rules {
        static RULES: std::sync::OnceLock<Rules> = std::sync::OnceLock::new();
        RULES
            .get_or_init(|| {
                [
                    teleparse::lexer::LexerRule::ignore(r#"^\s+"#),
                    teleparse::lexer::LexerRule::token(TokenType::Integer, r#"^\d+"#),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenType::Operator,
                        &["+", "-", "*", "/"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenType::Param,
                        &["(", ")"],
                    ),
                ]
            })
    }
    #[doc(hidden)]
    pub struct DerivedLexer<'s>(teleparse::lexer::LexerState<'s>, &'static Rules);
    impl<'s> teleparse::Lexer<'s> for DerivedLexer<'s> {
        type T = TokenType;
        #[inline]
        fn new(source: &'s str) -> Self {
            Self(teleparse::lexer::LexerState::new(source), derived_lexer_rules())
        }
        #[inline]
        fn next(
            &mut self,
        ) -> (Option<teleparse::Span>, Option<teleparse::Token<Self::T>>) {
            self.0.next(self.1)
        }
    }
};
