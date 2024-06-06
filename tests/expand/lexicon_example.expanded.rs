use teleparse::prelude::*;
#[repr(usize)]
pub enum TokenType {
    Integer = 0usize,
    Operator = 1usize,
    Param = 2usize,
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
/// Terminal symbol derived from [`TokenType`] with `terminal(Integer)`
#[automatically_derived]
pub struct Integer(pub teleparse::lex::Token<TokenType>);
#[automatically_derived]
impl ::core::fmt::Debug for Integer {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Integer", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Integer {
    #[inline]
    fn clone(&self) -> Integer {
        let _: ::core::clone::AssertParamIsClone<teleparse::lex::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Integer {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Integer {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Integer {
    #[inline]
    fn eq(&self, other: &Integer) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Integer {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::lex::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Integer {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for Integer {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    use teleparse::lex::Token;
    use teleparse::syntax::{
        AbstractSyntaxTree, First, FirstBuilder, FirstRel, Follow, FollowBuilder, Jump,
        Result as SynResult, Metadata,
    };
    use teleparse::{GrammarError, Parser};
    use ::std::borrow::Cow;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::string::String;
    use ::std::any::TypeId;
    #[automatically_derived]
    impl ::core::convert::From<Token<TokenType>> for Integer {
        #[inline]
        fn from(token: Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl AbstractSyntaxTree for Integer {
        type L = TokenType;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("Integer")
        }
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            let expr = FirstRel::insert_token(t, TokenType::Integer, None);
            builder.add(expr);
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn check_first_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_follow(_builder: &mut FollowBuilder<Self::L>) {}
        #[inline]
        fn check_first_follow_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _follow: &Follow<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_jump(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _jump: &mut Jump<Self::L>,
        ) {}
        /// Parse this AST node from the input stream
        #[inline]
        fn parse_ast<'s>(
            parser: &mut Parser<'s, Self::L>,
            meta: &Metadata<Self::L>,
        ) -> SynResult<Self, Self::L> {
            parser.parse_token(TokenType::Integer).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for Integer {
        type AST = Self;
        #[inline]
        fn from_ast<'s>(ast: Self, _: &mut Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(OpAdd = "+")`
#[automatically_derived]
pub struct OpAdd(pub teleparse::lex::Token<TokenType>);
#[automatically_derived]
impl ::core::fmt::Debug for OpAdd {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OpAdd", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for OpAdd {
    #[inline]
    fn clone(&self) -> OpAdd {
        let _: ::core::clone::AssertParamIsClone<teleparse::lex::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for OpAdd {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OpAdd {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OpAdd {
    #[inline]
    fn eq(&self, other: &OpAdd) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for OpAdd {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::lex::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for OpAdd {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for OpAdd {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    use teleparse::lex::Token;
    use teleparse::syntax::{
        AbstractSyntaxTree, First, FirstBuilder, FirstRel, Follow, FollowBuilder, Jump,
        Result as SynResult, Metadata,
    };
    use teleparse::{GrammarError, Parser};
    use ::std::borrow::Cow;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::string::String;
    use ::std::any::TypeId;
    #[automatically_derived]
    impl ::core::convert::From<Token<TokenType>> for OpAdd {
        #[inline]
        fn from(token: Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl AbstractSyntaxTree for OpAdd {
        type L = TokenType;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("OpAdd")
        }
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            let expr = FirstRel::insert_token(t, TokenType::Operator, Some("+"));
            builder.add(expr);
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn check_first_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_follow(_builder: &mut FollowBuilder<Self::L>) {}
        #[inline]
        fn check_first_follow_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _follow: &Follow<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_jump(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _jump: &mut Jump<Self::L>,
        ) {}
        /// Parse this AST node from the input stream
        #[inline]
        fn parse_ast<'s>(
            parser: &mut Parser<'s, Self::L>,
            meta: &Metadata<Self::L>,
        ) -> SynResult<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "+", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for OpAdd {
        type AST = Self;
        #[inline]
        fn from_ast<'s>(ast: Self, _: &mut Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(OpMul = "*")`
#[automatically_derived]
pub struct OpMul(pub teleparse::lex::Token<TokenType>);
#[automatically_derived]
impl ::core::fmt::Debug for OpMul {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OpMul", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for OpMul {
    #[inline]
    fn clone(&self) -> OpMul {
        let _: ::core::clone::AssertParamIsClone<teleparse::lex::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for OpMul {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OpMul {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OpMul {
    #[inline]
    fn eq(&self, other: &OpMul) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for OpMul {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::lex::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for OpMul {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for OpMul {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    use teleparse::lex::Token;
    use teleparse::syntax::{
        AbstractSyntaxTree, First, FirstBuilder, FirstRel, Follow, FollowBuilder, Jump,
        Result as SynResult, Metadata,
    };
    use teleparse::{GrammarError, Parser};
    use ::std::borrow::Cow;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::string::String;
    use ::std::any::TypeId;
    #[automatically_derived]
    impl ::core::convert::From<Token<TokenType>> for OpMul {
        #[inline]
        fn from(token: Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl AbstractSyntaxTree for OpMul {
        type L = TokenType;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("OpMul")
        }
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            let expr = FirstRel::insert_token(t, TokenType::Operator, Some("*"));
            builder.add(expr);
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn check_first_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_follow(_builder: &mut FollowBuilder<Self::L>) {}
        #[inline]
        fn check_first_follow_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _follow: &Follow<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_jump(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _jump: &mut Jump<Self::L>,
        ) {}
        /// Parse this AST node from the input stream
        #[inline]
        fn parse_ast<'s>(
            parser: &mut Parser<'s, Self::L>,
            meta: &Metadata<Self::L>,
        ) -> SynResult<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "*", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for OpMul {
        type AST = Self;
        #[inline]
        fn from_ast<'s>(ast: Self, _: &mut Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(ParamOpen = "(")`
#[automatically_derived]
pub struct ParamOpen(pub teleparse::lex::Token<TokenType>);
#[automatically_derived]
impl ::core::fmt::Debug for ParamOpen {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ParamOpen", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ParamOpen {
    #[inline]
    fn clone(&self) -> ParamOpen {
        let _: ::core::clone::AssertParamIsClone<teleparse::lex::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for ParamOpen {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ParamOpen {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ParamOpen {
    #[inline]
    fn eq(&self, other: &ParamOpen) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ParamOpen {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::lex::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for ParamOpen {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for ParamOpen {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    use teleparse::lex::Token;
    use teleparse::syntax::{
        AbstractSyntaxTree, First, FirstBuilder, FirstRel, Follow, FollowBuilder, Jump,
        Result as SynResult, Metadata,
    };
    use teleparse::{GrammarError, Parser};
    use ::std::borrow::Cow;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::string::String;
    use ::std::any::TypeId;
    #[automatically_derived]
    impl ::core::convert::From<Token<TokenType>> for ParamOpen {
        #[inline]
        fn from(token: Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl AbstractSyntaxTree for ParamOpen {
        type L = TokenType;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("ParamOpen")
        }
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            let expr = FirstRel::insert_token(t, TokenType::Param, Some("("));
            builder.add(expr);
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn check_first_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_follow(_builder: &mut FollowBuilder<Self::L>) {}
        #[inline]
        fn check_first_follow_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _follow: &Follow<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_jump(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _jump: &mut Jump<Self::L>,
        ) {}
        /// Parse this AST node from the input stream
        #[inline]
        fn parse_ast<'s>(
            parser: &mut Parser<'s, Self::L>,
            meta: &Metadata<Self::L>,
        ) -> SynResult<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Param, "(", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for ParamOpen {
        type AST = Self;
        #[inline]
        fn from_ast<'s>(ast: Self, _: &mut Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
/// Terminal symbol derived from [`TokenType`] with `terminal(ParamClose = ")")`
#[automatically_derived]
pub struct ParamClose(pub teleparse::lex::Token<TokenType>);
#[automatically_derived]
impl ::core::fmt::Debug for ParamClose {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ParamClose", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for ParamClose {
    #[inline]
    fn clone(&self) -> ParamClose {
        let _: ::core::clone::AssertParamIsClone<teleparse::lex::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for ParamClose {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ParamClose {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ParamClose {
    #[inline]
    fn eq(&self, other: &ParamClose) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for ParamClose {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::lex::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for ParamClose {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for ParamClose {
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
const _: () = {
    use teleparse::lex::Token;
    use teleparse::syntax::{
        AbstractSyntaxTree, First, FirstBuilder, FirstRel, Follow, FollowBuilder, Jump,
        Result as SynResult, Metadata,
    };
    use teleparse::{GrammarError, Parser};
    use ::std::borrow::Cow;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::string::String;
    use ::std::any::TypeId;
    #[automatically_derived]
    impl ::core::convert::From<Token<TokenType>> for ParamClose {
        #[inline]
        fn from(token: Token<TokenType>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl AbstractSyntaxTree for ParamClose {
        type L = TokenType;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("ParamClose")
        }
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            let expr = FirstRel::insert_token(t, TokenType::Param, Some(")"));
            builder.add(expr);
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn check_first_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_follow(_builder: &mut FollowBuilder<Self::L>) {}
        #[inline]
        fn check_first_follow_conflict(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _follow: &Follow<Self::L>,
        ) -> Result<(), GrammarError> {
            Ok(())
        }
        #[inline]
        fn build_jump(
            _seen: &mut BTreeSet<TypeId>,
            _first: &First<Self::L>,
            _jump: &mut Jump<Self::L>,
        ) {}
        /// Parse this AST node from the input stream
        #[inline]
        fn parse_ast<'s>(
            parser: &mut Parser<'s, Self::L>,
            meta: &Metadata<Self::L>,
        ) -> SynResult<Self, Self::L> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Param, ")", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for ParamClose {
        type AST = Self;
        #[inline]
        fn from_ast<'s>(ast: Self, _: &mut Parser<'s, TokenType>) -> Self {
            ast
        }
    }
};
const _: () = {
    use teleparse::Lexer as _;
    #[automatically_derived]
    impl teleparse::Lexicon for TokenType {
        type Bit = u8;
        type Lexer<'s> = teleparse::lex::LexerImpl<'s, Self>;
        type Map<T: Default + Clone> = [T; 3usize];
        #[inline]
        fn id(&self) -> usize {
            *self as usize
        }
        #[inline]
        fn from_id(id: usize) -> Self {
            unsafe { std::mem::transmute(id) }
        }
        #[inline]
        fn to_bit(&self) -> Self::Bit {
            (1 << self.id()) as Self::Bit
        }
        #[inline]
        fn first() -> Self {
            Self::Integer
        }
        fn next(&self) -> Option<Self> {
            match self {
                Self::Param => None,
                _ => {
                    let next = self.id() + 1;
                    Some(Self::from_id(next))
                }
            }
        }
        #[inline]
        fn should_extract(&self) -> bool {
            match self {
                _ => false,
            }
        }
        fn lexer<'s>(
            source: &'s str,
        ) -> Result<Self::Lexer<'s>, teleparse::GrammarError> {
            static RULES: ::std::sync::OnceLock<
                [teleparse::lex::Rule<TokenType>; 4usize],
            > = ::std::sync::OnceLock::new();
            let rules = RULES
                .get_or_init(|| {
                    [
                        teleparse::lex::Rule::ignore(r#"^\s+"#),
                        teleparse::lex::Rule::token(TokenType::Integer, r#"^\d+"#),
                        teleparse::lex::Rule::token_literal(
                            TokenType::Operator,
                            &["+", "*"],
                        ),
                        teleparse::lex::Rule::token_literal(
                            TokenType::Param,
                            &["(", ")"],
                        ),
                    ]
                });
            Ok(teleparse::lex::LexerImpl::new(source, rules)?)
        }
    }
};
