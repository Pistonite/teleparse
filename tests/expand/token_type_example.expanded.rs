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
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(Integer)`
#[automatically_derived]
pub struct Integer(pub teleparse::Token<TokenType>);
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
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
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
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
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
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{LL1Error, AstResult, ToSpan, Span, Token, Parser, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::root::RootMetadata;
    use teleparse::table::first::{FirstBuilder, FirstExpr, First};
    use teleparse::table::follow::{FollowBuilder, Follow};
    use teleparse::table::parsing::Parsing;
    use ::core::any::TypeId;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::option::Option;
    use ::std::borrow::Cow;
    impl SyntaxTree for Integer {
        type T = TokenType;
        type AST = Token<TokenType>;
        #[inline]
        fn type_id() -> TypeId {
            TypeId::of::<Self>()
        }
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("Integer")
        }
        #[inline]
        fn produces_epsilon() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _set: &mut BTreeSet<TypeId>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        fn build_first(builder: &mut FirstBuilder<Self::T>) {
            let t = Self::type_id();
            let expr = FirstExpr::insert_token(t, TokenType::Integer, None);
            builder.add(expr);
        }
        #[inline]
        fn check_first_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::T>) {}
        #[inline]
        fn check_first_follow_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
            follow: &Follow<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
            seen.insert(Self::type_id());
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            meta: &RootMetadata<Self::T>,
        ) -> AstResult<Self::T, Self::AST> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token(TokenType::Integer)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpAdd = "+")`
#[automatically_derived]
pub struct OpAdd(pub teleparse::Token<TokenType>);
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
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
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
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
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
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{LL1Error, AstResult, ToSpan, Span, Token, Parser, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::root::RootMetadata;
    use teleparse::table::first::{FirstBuilder, FirstExpr, First};
    use teleparse::table::follow::{FollowBuilder, Follow};
    use teleparse::table::parsing::Parsing;
    use ::core::any::TypeId;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::option::Option;
    use ::std::borrow::Cow;
    impl SyntaxTree for OpAdd {
        type T = TokenType;
        type AST = Token<TokenType>;
        #[inline]
        fn type_id() -> TypeId {
            TypeId::of::<Self>()
        }
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("OpAdd")
        }
        #[inline]
        fn produces_epsilon() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _set: &mut BTreeSet<TypeId>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        fn build_first(builder: &mut FirstBuilder<Self::T>) {
            let t = Self::type_id();
            let expr = FirstExpr::insert_token(t, TokenType::Operator, Some("+"));
            builder.add(expr);
        }
        #[inline]
        fn check_first_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::T>) {}
        #[inline]
        fn check_first_follow_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
            follow: &Follow<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
            seen.insert(Self::type_id());
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            meta: &RootMetadata<Self::T>,
        ) -> AstResult<Self::T, Self::AST> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "+", follow)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpSub = "-")`
#[automatically_derived]
pub struct OpSub(pub teleparse::Token<TokenType>);
#[automatically_derived]
impl ::core::fmt::Debug for OpSub {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OpSub", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for OpSub {
    #[inline]
    fn clone(&self) -> OpSub {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for OpSub {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OpSub {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OpSub {
    #[inline]
    fn eq(&self, other: &OpSub) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for OpSub {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for OpSub {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for OpSub {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{LL1Error, AstResult, ToSpan, Span, Token, Parser, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::root::RootMetadata;
    use teleparse::table::first::{FirstBuilder, FirstExpr, First};
    use teleparse::table::follow::{FollowBuilder, Follow};
    use teleparse::table::parsing::Parsing;
    use ::core::any::TypeId;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::option::Option;
    use ::std::borrow::Cow;
    impl SyntaxTree for OpSub {
        type T = TokenType;
        type AST = Token<TokenType>;
        #[inline]
        fn type_id() -> TypeId {
            TypeId::of::<Self>()
        }
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("OpSub")
        }
        #[inline]
        fn produces_epsilon() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _set: &mut BTreeSet<TypeId>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        fn build_first(builder: &mut FirstBuilder<Self::T>) {
            let t = Self::type_id();
            let expr = FirstExpr::insert_token(t, TokenType::Operator, Some("-"));
            builder.add(expr);
        }
        #[inline]
        fn check_first_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::T>) {}
        #[inline]
        fn check_first_follow_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
            follow: &Follow<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
            seen.insert(Self::type_id());
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            meta: &RootMetadata<Self::T>,
        ) -> AstResult<Self::T, Self::AST> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "-", follow)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpMul = "*")`
#[automatically_derived]
pub struct OpMul(pub teleparse::Token<TokenType>);
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
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
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
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
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
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{LL1Error, AstResult, ToSpan, Span, Token, Parser, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::root::RootMetadata;
    use teleparse::table::first::{FirstBuilder, FirstExpr, First};
    use teleparse::table::follow::{FollowBuilder, Follow};
    use teleparse::table::parsing::Parsing;
    use ::core::any::TypeId;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::option::Option;
    use ::std::borrow::Cow;
    impl SyntaxTree for OpMul {
        type T = TokenType;
        type AST = Token<TokenType>;
        #[inline]
        fn type_id() -> TypeId {
            TypeId::of::<Self>()
        }
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("OpMul")
        }
        #[inline]
        fn produces_epsilon() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _set: &mut BTreeSet<TypeId>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        fn build_first(builder: &mut FirstBuilder<Self::T>) {
            let t = Self::type_id();
            let expr = FirstExpr::insert_token(t, TokenType::Operator, Some("*"));
            builder.add(expr);
        }
        #[inline]
        fn check_first_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::T>) {}
        #[inline]
        fn check_first_follow_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
            follow: &Follow<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
            seen.insert(Self::type_id());
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            meta: &RootMetadata<Self::T>,
        ) -> AstResult<Self::T, Self::AST> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "*", follow)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(OpDiv = "/")`
#[automatically_derived]
pub struct OpDiv(pub teleparse::Token<TokenType>);
#[automatically_derived]
impl ::core::fmt::Debug for OpDiv {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OpDiv", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for OpDiv {
    #[inline]
    fn clone(&self) -> OpDiv {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for OpDiv {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for OpDiv {}
#[automatically_derived]
impl ::core::cmp::PartialEq for OpDiv {
    #[inline]
    fn eq(&self, other: &OpDiv) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for OpDiv {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for OpDiv {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for OpDiv {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{LL1Error, AstResult, ToSpan, Span, Token, Parser, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::root::RootMetadata;
    use teleparse::table::first::{FirstBuilder, FirstExpr, First};
    use teleparse::table::follow::{FollowBuilder, Follow};
    use teleparse::table::parsing::Parsing;
    use ::core::any::TypeId;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::option::Option;
    use ::std::borrow::Cow;
    impl SyntaxTree for OpDiv {
        type T = TokenType;
        type AST = Token<TokenType>;
        #[inline]
        fn type_id() -> TypeId {
            TypeId::of::<Self>()
        }
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("OpDiv")
        }
        #[inline]
        fn produces_epsilon() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _set: &mut BTreeSet<TypeId>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        fn build_first(builder: &mut FirstBuilder<Self::T>) {
            let t = Self::type_id();
            let expr = FirstExpr::insert_token(t, TokenType::Operator, Some("/"));
            builder.add(expr);
        }
        #[inline]
        fn check_first_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::T>) {}
        #[inline]
        fn check_first_follow_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
            follow: &Follow<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
            seen.insert(Self::type_id());
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            meta: &RootMetadata<Self::T>,
        ) -> AstResult<Self::T, Self::AST> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Operator, "/", follow)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(ParamOpen = "(")`
#[automatically_derived]
pub struct ParamOpen(pub teleparse::Token<TokenType>);
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
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
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
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
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
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{LL1Error, AstResult, ToSpan, Span, Token, Parser, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::root::RootMetadata;
    use teleparse::table::first::{FirstBuilder, FirstExpr, First};
    use teleparse::table::follow::{FollowBuilder, Follow};
    use teleparse::table::parsing::Parsing;
    use ::core::any::TypeId;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::option::Option;
    use ::std::borrow::Cow;
    impl SyntaxTree for ParamOpen {
        type T = TokenType;
        type AST = Token<TokenType>;
        #[inline]
        fn type_id() -> TypeId {
            TypeId::of::<Self>()
        }
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("ParamOpen")
        }
        #[inline]
        fn produces_epsilon() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _set: &mut BTreeSet<TypeId>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        fn build_first(builder: &mut FirstBuilder<Self::T>) {
            let t = Self::type_id();
            let expr = FirstExpr::insert_token(t, TokenType::Param, Some("("));
            builder.add(expr);
        }
        #[inline]
        fn check_first_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::T>) {}
        #[inline]
        fn check_first_follow_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
            follow: &Follow<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
            seen.insert(Self::type_id());
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            meta: &RootMetadata<Self::T>,
        ) -> AstResult<Self::T, Self::AST> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Param, "(", follow)
        }
    }
};
/// SyntaxTree terminal derived from [`TokenType`] with `terminal(ParamClose = ")")`
#[automatically_derived]
pub struct ParamClose(pub teleparse::Token<TokenType>);
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
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<TokenType>>;
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
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<TokenType>>;
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
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
const _: () = {
    use teleparse::{LL1Error, AstResult, ToSpan, Span, Token, Parser, SyntaxTree};
    use teleparse::parser::ParserState;
    use teleparse::root::RootMetadata;
    use teleparse::table::first::{FirstBuilder, FirstExpr, First};
    use teleparse::table::follow::{FollowBuilder, Follow};
    use teleparse::table::parsing::Parsing;
    use ::core::any::TypeId;
    use ::std::vec::Vec;
    use ::std::collections::BTreeSet;
    use ::std::option::Option;
    use ::std::borrow::Cow;
    impl SyntaxTree for ParamClose {
        type T = TokenType;
        type AST = Token<TokenType>;
        #[inline]
        fn type_id() -> TypeId {
            TypeId::of::<Self>()
        }
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("ParamClose")
        }
        #[inline]
        fn produces_epsilon() -> bool {
            false
        }
        #[inline]
        fn check_left_recursive(
            _stack: &mut Vec<String>,
            _set: &mut BTreeSet<TypeId>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        fn build_first(builder: &mut FirstBuilder<Self::T>) {
            let t = Self::type_id();
            let expr = FirstExpr::insert_token(t, TokenType::Param, Some(")"));
            builder.add(expr);
        }
        #[inline]
        fn check_first_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_follow(builder: &mut FollowBuilder<Self::T>) {}
        #[inline]
        fn check_first_follow_conflict_recursive(
            seen: &mut BTreeSet<TypeId>,
            first: &First<Self::T>,
            follow: &Follow<Self::T>,
        ) -> Result<(), LL1Error> {
            Ok(())
        }
        #[inline]
        fn build_parsing(seen: &mut BTreeSet<TypeId>, parsing: &mut Parsing<Self::T>) {
            seen.insert(Self::type_id());
        }
        #[inline]
        fn try_parse_ast<'s>(
            parser: &mut Parser<'s, Self::T>,
            meta: &RootMetadata<Self::T>,
        ) -> AstResult<Self::T, Self::AST> {
            let follow = meta.follow.get(&Self::type_id());
            parser.parse_token_lit(TokenType::Param, ")", follow)
        }
    }
};
#[automatically_derived]
const _: () = {
    use teleparse::Lexer as _;
    impl teleparse::TokenType for TokenType {
        type Bit = u8;
        type Lexer<'s> = DerivedLexer<'s>;
        type Map<T: Default + Clone> = [T; 3usize];
        type Ctx = ();
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
