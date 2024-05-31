use teleparse::prelude::*;
#[repr(usize)]
pub enum TokenU32 {
    X00000 = 0usize,
    X00001 = 1usize,
    X00010 = 2usize,
    X00011 = 3usize,
    X00100 = 4usize,
    X00101 = 5usize,
    X00110 = 6usize,
    X00111 = 7usize,
    X01000 = 8usize,
    X01001 = 9usize,
    X01010 = 10usize,
    X01011 = 11usize,
    X01100 = 12usize,
    X01101 = 13usize,
    X01110 = 14usize,
    X01111 = 15usize,
    X10000 = 16usize,
    X10001 = 17usize,
    X10010 = 18usize,
    X10011 = 19usize,
    X10100 = 20usize,
    X10101 = 21usize,
    X10110 = 22usize,
    X10111 = 23usize,
    X11000 = 24usize,
    X11001 = 25usize,
    X11010 = 26usize,
    X11011 = 27usize,
    X11100 = 28usize,
    X11101 = 29usize,
    X11110 = 30usize,
    X11111 = 31usize,
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
impl ::core::fmt::Debug for X00000 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00000", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00000 {
    #[inline]
    fn clone(&self) -> X00000 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00000 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00000 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00000 {
    #[inline]
    fn eq(&self, other: &X00000) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00000 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00000 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00000 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00000 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00000, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00000, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00001 = "0000")`
#[automatically_derived]
pub struct X00001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X00001 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00001", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00001 {
    #[inline]
    fn clone(&self) -> X00001 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00001 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00001 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00001 {
    #[inline]
    fn eq(&self, other: &X00001) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00001 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00001 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00001 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00001 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00001, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00001, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00010 = "0000")`
#[automatically_derived]
pub struct X00010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X00010 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00010", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00010 {
    #[inline]
    fn clone(&self) -> X00010 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00010 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00010 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00010 {
    #[inline]
    fn eq(&self, other: &X00010) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00010 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00010 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00010 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00010 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00010, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00010, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00011 = "0000")`
#[automatically_derived]
pub struct X00011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X00011 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00011", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00011 {
    #[inline]
    fn clone(&self) -> X00011 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00011 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00011 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00011 {
    #[inline]
    fn eq(&self, other: &X00011) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00011 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00011 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00011 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00011 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00011, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00011, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00100 = "0000")`
#[automatically_derived]
pub struct X00100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X00100 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00100", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00100 {
    #[inline]
    fn clone(&self) -> X00100 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00100 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00100 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00100 {
    #[inline]
    fn eq(&self, other: &X00100) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00100 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00100 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00100 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00100 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00100, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00100, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00101 = "0000")`
#[automatically_derived]
pub struct X00101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X00101 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00101", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00101 {
    #[inline]
    fn clone(&self) -> X00101 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00101 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00101 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00101 {
    #[inline]
    fn eq(&self, other: &X00101) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00101 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00101 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00101 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00101 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00101, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00101, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00110 = "0000")`
#[automatically_derived]
pub struct X00110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X00110 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00110", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00110 {
    #[inline]
    fn clone(&self) -> X00110 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00110 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00110 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00110 {
    #[inline]
    fn eq(&self, other: &X00110) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00110 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00110 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00110 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00110 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00110, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00110, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X00111 = "0000")`
#[automatically_derived]
pub struct X00111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X00111 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X00111", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X00111 {
    #[inline]
    fn clone(&self) -> X00111 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X00111 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X00111 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X00111 {
    #[inline]
    fn eq(&self, other: &X00111) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X00111 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X00111 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X00111 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X00111 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X00111, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X00111, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01000 = "0000")`
#[automatically_derived]
pub struct X01000(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01000 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01000", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01000 {
    #[inline]
    fn clone(&self) -> X01000 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01000 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01000 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01000 {
    #[inline]
    fn eq(&self, other: &X01000) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01000 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01000 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01000 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01000 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01000, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01000, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01001 = "0000")`
#[automatically_derived]
pub struct X01001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01001 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01001", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01001 {
    #[inline]
    fn clone(&self) -> X01001 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01001 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01001 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01001 {
    #[inline]
    fn eq(&self, other: &X01001) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01001 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01001 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01001 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01001 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01001, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01001, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01010 = "0000")`
#[automatically_derived]
pub struct X01010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01010 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01010", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01010 {
    #[inline]
    fn clone(&self) -> X01010 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01010 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01010 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01010 {
    #[inline]
    fn eq(&self, other: &X01010) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01010 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01010 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01010 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01010 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01010, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01010, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01011 = "0000")`
#[automatically_derived]
pub struct X01011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01011 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01011", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01011 {
    #[inline]
    fn clone(&self) -> X01011 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01011 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01011 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01011 {
    #[inline]
    fn eq(&self, other: &X01011) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01011 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01011 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01011 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01011 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01011, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01011, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01100 = "0000")`
#[automatically_derived]
pub struct X01100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01100 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01100", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01100 {
    #[inline]
    fn clone(&self) -> X01100 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01100 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01100 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01100 {
    #[inline]
    fn eq(&self, other: &X01100) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01100 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01100 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01100 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01100 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01100, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01100, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01101 = "0000")`
#[automatically_derived]
pub struct X01101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01101 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01101", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01101 {
    #[inline]
    fn clone(&self) -> X01101 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01101 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01101 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01101 {
    #[inline]
    fn eq(&self, other: &X01101) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01101 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01101 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01101 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01101 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01101, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01101, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01110 = "0000")`
#[automatically_derived]
pub struct X01110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01110 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01110", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01110 {
    #[inline]
    fn clone(&self) -> X01110 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01110 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01110 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01110 {
    #[inline]
    fn eq(&self, other: &X01110) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01110 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01110 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01110 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01110 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01110, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01110, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X01111 = "0000")`
#[automatically_derived]
pub struct X01111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X01111 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X01111", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X01111 {
    #[inline]
    fn clone(&self) -> X01111 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X01111 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X01111 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X01111 {
    #[inline]
    fn eq(&self, other: &X01111) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X01111 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X01111 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X01111 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X01111 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X01111, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X01111, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10000 = "0000")`
#[automatically_derived]
pub struct X10000(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10000 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10000", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10000 {
    #[inline]
    fn clone(&self) -> X10000 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10000 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10000 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10000 {
    #[inline]
    fn eq(&self, other: &X10000) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10000 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10000 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10000 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10000 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10000, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10000, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10001 = "0000")`
#[automatically_derived]
pub struct X10001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10001 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10001", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10001 {
    #[inline]
    fn clone(&self) -> X10001 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10001 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10001 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10001 {
    #[inline]
    fn eq(&self, other: &X10001) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10001 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10001 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10001 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10001 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10001, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10001, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10010 = "0000")`
#[automatically_derived]
pub struct X10010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10010 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10010", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10010 {
    #[inline]
    fn clone(&self) -> X10010 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10010 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10010 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10010 {
    #[inline]
    fn eq(&self, other: &X10010) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10010 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10010 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10010 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10010 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10010, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10010, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10011 = "0000")`
#[automatically_derived]
pub struct X10011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10011 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10011", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10011 {
    #[inline]
    fn clone(&self) -> X10011 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10011 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10011 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10011 {
    #[inline]
    fn eq(&self, other: &X10011) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10011 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10011 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10011 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10011 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10011, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10011, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10100 = "0000")`
#[automatically_derived]
pub struct X10100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10100 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10100", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10100 {
    #[inline]
    fn clone(&self) -> X10100 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10100 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10100 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10100 {
    #[inline]
    fn eq(&self, other: &X10100) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10100 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10100 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10100 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10100 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10100, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10100, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10101 = "0000")`
#[automatically_derived]
pub struct X10101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10101 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10101", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10101 {
    #[inline]
    fn clone(&self) -> X10101 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10101 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10101 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10101 {
    #[inline]
    fn eq(&self, other: &X10101) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10101 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10101 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10101 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10101 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10101, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10101, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10110 = "0000")`
#[automatically_derived]
pub struct X10110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10110 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10110", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10110 {
    #[inline]
    fn clone(&self) -> X10110 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10110 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10110 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10110 {
    #[inline]
    fn eq(&self, other: &X10110) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10110 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10110 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10110 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10110 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10110, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10110, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X10111 = "0000")`
#[automatically_derived]
pub struct X10111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X10111 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X10111", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X10111 {
    #[inline]
    fn clone(&self) -> X10111 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X10111 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X10111 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X10111 {
    #[inline]
    fn eq(&self, other: &X10111) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X10111 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X10111 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X10111 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X10111 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X10111, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X10111, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11000 = "0000")`
#[automatically_derived]
pub struct X11000(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11000 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11000", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11000 {
    #[inline]
    fn clone(&self) -> X11000 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11000 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11000 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11000 {
    #[inline]
    fn eq(&self, other: &X11000) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11000 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11000 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11000 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11000 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11000, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11000, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11001 = "0000")`
#[automatically_derived]
pub struct X11001(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11001 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11001", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11001 {
    #[inline]
    fn clone(&self) -> X11001 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11001 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11001 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11001 {
    #[inline]
    fn eq(&self, other: &X11001) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11001 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11001 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11001 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11001 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11001, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11001, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11010 = "0000")`
#[automatically_derived]
pub struct X11010(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11010 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11010", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11010 {
    #[inline]
    fn clone(&self) -> X11010 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11010 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11010 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11010 {
    #[inline]
    fn eq(&self, other: &X11010) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11010 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11010 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11010 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11010 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11010, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11010, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11011 = "0000")`
#[automatically_derived]
pub struct X11011(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11011 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11011", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11011 {
    #[inline]
    fn clone(&self) -> X11011 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11011 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11011 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11011 {
    #[inline]
    fn eq(&self, other: &X11011) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11011 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11011 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11011 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11011 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11011, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11011, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11100 = "0000")`
#[automatically_derived]
pub struct X11100(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11100 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11100", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11100 {
    #[inline]
    fn clone(&self) -> X11100 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11100 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11100 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11100 {
    #[inline]
    fn eq(&self, other: &X11100) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11100 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11100 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11100 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11100 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11100, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11100, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11101 = "0000")`
#[automatically_derived]
pub struct X11101(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11101 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11101", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11101 {
    #[inline]
    fn clone(&self) -> X11101 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11101 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11101 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11101 {
    #[inline]
    fn eq(&self, other: &X11101) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11101 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11101 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11101 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11101 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11101, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11101, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11110 = "0000")`
#[automatically_derived]
pub struct X11110(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11110 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11110", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11110 {
    #[inline]
    fn clone(&self) -> X11110 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11110 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11110 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11110 {
    #[inline]
    fn eq(&self, other: &X11110) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11110 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11110 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11110 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11110 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11110, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11110, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenU32`] with `terminal(X11111 = "0000")`
#[automatically_derived]
pub struct X11111(pub teleparse::Token<TokenU32>);
#[automatically_derived]
impl ::core::fmt::Debug for X11111 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X11111", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for X11111 {
    #[inline]
    fn clone(&self) -> X11111 {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenU32>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for X11111 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for X11111 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for X11111 {
    #[inline]
    fn eq(&self, other: &X11111) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for X11111 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenU32>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for X11111 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for X11111 {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{ToSpan, Span, Token, Parser, SyntaxResult, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::table::{SyntaxTreeTable, LitTable, TermSet};
    use core::ops::Deref;
    impl SyntaxTree for X11111 {
        type T = TokenU32;
        type AST = Token<TokenU32>;
        #[inline]
        fn type_id() -> ::core::any::TypeId {
            ::core::any::TypeId::of::<Self>()
        }
        #[inline]
        fn can_be_empty() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut ::std::vec::Vec<::core::any::TypeId>,
            _set: &mut ::std::collections::BTreeSet<::core::any::TypeId>,
        ) -> bool {
            false
        }
        fn build_first_table(
            s_table: &mut SyntaxTreeTable<Self::T>,
            lits: &mut LitTable,
        ) -> bool {
            let t = Self::type_id();
            s_table
                .init(
                    t,
                    |_| {
                        let mut set = TermSet::default();
                        let lit = lits.get_or_add("0000");
                        set.insert_token_type_match(TokenU32::X11111, lit);
                        (set, true)
                    },
                )
        }
        fn build_follow_table<'s>(
            s_table: &'s SyntaxTreeTable<Self::T>,
            f_table: &mut SyntaxTreeTable<Self::T>,
            follows: &TermSet<Self::T>,
        ) -> (::std::borrow::Cow<'s, TermSet<Self::T>>, bool) {
            let t = Self::type_id();
            f_table.get_mut(t).union(follows);
            (s_table.get(t), true)
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            f_table: &SyntaxTreeTable<Self::T>,
            _should_recover: bool,
        ) -> SyntaxResult<Self::T, Self::AST> {
            let t = Self::type_id();
            let f = f_table.get(t);
            let follows = f.deref();
            let result = parser.parse_token_match(TokenU32::X11111, follows, "0000");
            match result {
                Ok(ast) => Ok(ast),
                Err(e) => e.into(),
            }
        }
        #[inline]
        fn into_parse_tree<'s>(
            ast: Self::AST,
            _parser: &mut Parser<'s, Self::T>,
        ) -> Self {
            Self(ast)
        }
    }
};
#[automatically_derived]
const _: () = {
    use teleparse::Lexer as _;
    impl teleparse::TokenType for TokenU32 {
        type Bit = u32;
        type Lexer<'s> = DerivedLexer<'s>;
        type Follow = [teleparse::table::LitSet; 32usize];
        type Ctx = ();
        #[inline]
        fn id(&self) -> usize {
            *self as usize
        }
        #[inline]
        fn to_bit(&self) -> Self::Bit {
            (1 << self.id()) as Self::Bit
        }
        #[inline]
        fn first() -> Self {
            Self::X00000
        }
        fn next(&self) -> Option<Self> {
            match self {
                Self::X11111 => None,
                _ => {
                    let next = self.id() + 1;
                    Some(unsafe { std::mem::transmute(next) })
                }
            }
        }
        #[inline]
        fn should_extract(&self) -> bool {
            match self {
                _ => false,
            }
        }
        #[inline]
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
        fn next(
            &mut self,
        ) -> (Option<teleparse::Span>, Option<teleparse::Token<Self::T>>) {
            self.0.next(self.1)
        }
    }
    #[doc(hidden)]
    impl<'s> DerivedLexer<'s> {
        #[inline]
        fn new(source: &'s str) -> Self {
            Self(teleparse::lexer::LexerState::new(source), derived_lexer_rules())
        }
    }
};
