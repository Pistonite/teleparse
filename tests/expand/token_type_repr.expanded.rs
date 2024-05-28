use teleparse::prelude::*;
#[repr(u32)]
pub enum TokenU32 {
    X00000 = 0x1u32,
    X00001 = 0x2u32,
    X00010 = 0x4u32,
    X00011 = 0x8u32,
    X00100 = 0x10u32,
    X00101 = 0x20u32,
    X00110 = 0x40u32,
    X00111 = 0x80u32,
    X01000 = 0x100u32,
    X01001 = 0x200u32,
    X01010 = 0x400u32,
    X01011 = 0x800u32,
    X01100 = 0x1000u32,
    X01101 = 0x2000u32,
    X01110 = 0x4000u32,
    X01111 = 0x8000u32,
    X10000 = 0x10000u32,
    X10001 = 0x20000u32,
    X10010 = 0x40000u32,
    X10011 = 0x80000u32,
    X10100 = 0x100000u32,
    X10101 = 0x200000u32,
    X10110 = 0x400000u32,
    X10111 = 0x800000u32,
    X11000 = 0x1000000u32,
    X11001 = 0x2000000u32,
    X11010 = 0x4000000u32,
    X11011 = 0x8000000u32,
    X11100 = 0x10000000u32,
    X11101 = 0x20000000u32,
    X11110 = 0x40000000u32,
    X11111 = 0x80000000u32,
}
#[automatically_derived]
impl ::core::fmt::Debug for TokenU32 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TokenU32::X00000 => "X00000",
                TokenU32::X00001 => "X00001",
                TokenU32::X00010 => "X00010",
                TokenU32::X00011 => "X00011",
                TokenU32::X00100 => "X00100",
                TokenU32::X00101 => "X00101",
                TokenU32::X00110 => "X00110",
                TokenU32::X00111 => "X00111",
                TokenU32::X01000 => "X01000",
                TokenU32::X01001 => "X01001",
                TokenU32::X01010 => "X01010",
                TokenU32::X01011 => "X01011",
                TokenU32::X01100 => "X01100",
                TokenU32::X01101 => "X01101",
                TokenU32::X01110 => "X01110",
                TokenU32::X01111 => "X01111",
                TokenU32::X10000 => "X10000",
                TokenU32::X10001 => "X10001",
                TokenU32::X10010 => "X10010",
                TokenU32::X10011 => "X10011",
                TokenU32::X10100 => "X10100",
                TokenU32::X10101 => "X10101",
                TokenU32::X10110 => "X10110",
                TokenU32::X10111 => "X10111",
                TokenU32::X11000 => "X11000",
                TokenU32::X11001 => "X11001",
                TokenU32::X11010 => "X11010",
                TokenU32::X11011 => "X11011",
                TokenU32::X11100 => "X11100",
                TokenU32::X11101 => "X11101",
                TokenU32::X11110 => "X11110",
                TokenU32::X11111 => "X11111",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TokenU32 {
    #[inline]
    fn clone(&self) -> TokenU32 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for TokenU32 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TokenU32 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TokenU32 {
    #[inline]
    fn eq(&self, other: &TokenU32) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TokenU32 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for TokenU32 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00000 = "0000")`
#[automatically_derived]
pub struct X00000(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00000 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00000, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00001 = "0000")`
#[automatically_derived]
pub struct X00001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00001 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00001, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00010 = "0000")`
#[automatically_derived]
pub struct X00010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00010 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00010, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00011 = "0000")`
#[automatically_derived]
pub struct X00011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00011 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00011, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00100 = "0000")`
#[automatically_derived]
pub struct X00100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00100 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00100, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00101 = "0000")`
#[automatically_derived]
pub struct X00101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00101 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00101, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00110 = "0000")`
#[automatically_derived]
pub struct X00110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00110 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00110, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00111 = "0000")`
#[automatically_derived]
pub struct X00111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X00111 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X00111, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01000 = "0000")`
#[automatically_derived]
pub struct X01000(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01000 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01000, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01001 = "0000")`
#[automatically_derived]
pub struct X01001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01001 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01001, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01010 = "0000")`
#[automatically_derived]
pub struct X01010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01010 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01010, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01011 = "0000")`
#[automatically_derived]
pub struct X01011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01011 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01011, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01100 = "0000")`
#[automatically_derived]
pub struct X01100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01100 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01100, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01101 = "0000")`
#[automatically_derived]
pub struct X01101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01101 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01101, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01110 = "0000")`
#[automatically_derived]
pub struct X01110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01110 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01110, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01111 = "0000")`
#[automatically_derived]
pub struct X01111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X01111 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X01111, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10000 = "0000")`
#[automatically_derived]
pub struct X10000(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10000 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10000, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10001 = "0000")`
#[automatically_derived]
pub struct X10001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10001 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10001, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10010 = "0000")`
#[automatically_derived]
pub struct X10010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10010 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10010, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10011 = "0000")`
#[automatically_derived]
pub struct X10011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10011 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10011, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10100 = "0000")`
#[automatically_derived]
pub struct X10100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10100 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10100, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10101 = "0000")`
#[automatically_derived]
pub struct X10101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10101 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10101, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10110 = "0000")`
#[automatically_derived]
pub struct X10110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10110 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10110, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10111 = "0000")`
#[automatically_derived]
pub struct X10111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X10111 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X10111, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11000 = "0000")`
#[automatically_derived]
pub struct X11000(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11000 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11000, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11001 = "0000")`
#[automatically_derived]
pub struct X11001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11001 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11001, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11010 = "0000")`
#[automatically_derived]
pub struct X11010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11010 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11010, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11011 = "0000")`
#[automatically_derived]
pub struct X11011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11011 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11011, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11100 = "0000")`
#[automatically_derived]
pub struct X11100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11100 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11100, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11101 = "0000")`
#[automatically_derived]
pub struct X11101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11101 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11101, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11110 = "0000")`
#[automatically_derived]
pub struct X11110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11110 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11110, "0000", parser)
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
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11111 = "0000")`
#[automatically_derived]
pub struct X11111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
const _: () = {
    use teleparse::{Token, Lexer, Parser, SyntaxResult, SyntaxTree};
    impl SyntaxTree for X11111 {
        type T = TokenU32;
        type Ctx = ();
        type AST = Token<TokenU32>;
        #[inline]
        fn span_of(ast: &Self::AST) -> Span {
            ast.span
        }
        #[inline]
        fn try_parse_ast<'s, L: Lexer<'s, T = Self::T>>(
            parser: &mut Parser<'s, Self::T, L, Self::Ctx>,
        ) -> SyntaxResult<Self::AST> {
            teleparse::imp::token::parse_token_match(TokenU32::X11111, "0000", parser)
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
    impl teleparse::TokenType for TokenU32 {
        type Repr = u32;
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
            Self::X00000
        }
        fn next(&self) -> Option<Self> {
            match self {
                Self::X11111 => None,
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
    type Rules = [teleparse::lexer::LexerRule<TokenU32>; 32usize];
    #[doc(hidden)]
    fn derived_lexer_rules() -> &'static Rules {
        static RULES: std::sync::OnceLock<Rules> = std::sync::OnceLock::new();
        RULES
            .get_or_init(|| {
                [
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00000,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00001,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00010,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00011,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00100,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00101,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00110,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X00111,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01000,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01001,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01010,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01011,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01100,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01101,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01110,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X01111,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10000,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10001,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10010,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10011,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10100,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10101,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10110,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X10111,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11000,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11001,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11010,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11011,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11100,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11101,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11110,
                        &["0000"],
                    ),
                    teleparse::lexer::LexerRule::token_literal(
                        TokenU32::X11111,
                        &["0000"],
                    ),
                ]
            })
    }
    #[doc(hidden)]
    pub struct DerivedLexer<'s>(teleparse::lexer::LexerState<'s>, &'static Rules);
    impl<'s> teleparse::Lexer<'s> for DerivedLexer<'s> {
        type T = TokenU32;
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
