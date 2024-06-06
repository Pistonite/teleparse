use teleparse::prelude::*;
#[repr(usize)]
pub enum MyToken {
    Food = 0usize,
}
#[automatically_derived]
impl ::core::fmt::Debug for MyToken {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Food")
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MyToken {
    #[inline]
    fn clone(&self) -> MyToken {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for MyToken {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MyToken {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MyToken {
    #[inline]
    fn eq(&self, other: &MyToken) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for MyToken {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for MyToken {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
}
/// Terminal symbol derived from [`MyToken`] with `terminal(Pizza = "pizza")`
#[automatically_derived]
pub struct Pizza(pub teleparse::lex::Token<MyToken>);
#[automatically_derived]
impl ::core::fmt::Debug for Pizza {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Pizza", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Pizza {
    #[inline]
    fn clone(&self) -> Pizza {
        let _: ::core::clone::AssertParamIsClone<teleparse::lex::Token<MyToken>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Pizza {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Pizza {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Pizza {
    #[inline]
    fn eq(&self, other: &Pizza) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Pizza {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::lex::Token<MyToken>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Pizza {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for Pizza {
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
    impl ::core::convert::From<Token<MyToken>> for Pizza {
        #[inline]
        fn from(token: Token<MyToken>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl AbstractSyntaxTree for Pizza {
        type L = MyToken;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("Pizza")
        }
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            let expr = FirstRel::insert_token(t, MyToken::Food, Some("pizza"));
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
            parser.parse_token_lit(MyToken::Food, "pizza", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for Pizza {
        type AST = Self;
        #[inline]
        fn from_ast<'s>(ast: Self, _: &mut Parser<'s, MyToken>) -> Self {
            ast
        }
    }
};
const _: () = {
    use ::std::vec::Vec;
    use ::std::result::Result;
    use ::std::collections::BTreeSet;
    use ::std::sync::OnceLock;
    use teleparse::syntax::{FirstBuilder, FollowBuilder, Jump, Metadata};
    use teleparse::GrammarError;
    #[automatically_derived]
    impl teleparse::AbstractSyntaxRoot for Pizza {
        fn metadata() -> &'static Result<Metadata<MyToken>, GrammarError> {
            static METADATA: OnceLock<Result<Metadata<MyToken>, GrammarError>> = OnceLock::new();
            METADATA
                .get_or_init(|| {
                    let mut first = FirstBuilder::new();
                    Self::build_first(&mut first);
                    let first = first.build();
                    let mut stack = Vec::new();
                    let mut seen = BTreeSet::new();
                    Self::check_left_recursive(&mut stack, &mut seen, &first)?;
                    seen.clear();
                    Self::check_first_conflict(&mut seen, &first)?;
                    seen.clear();
                    let mut follow = FollowBuilder::new(first);
                    Self::build_follow(&mut follow);
                    let (first, follow) = follow.build(<Pizza>::type_id());
                    Self::check_first_follow_conflict(&mut seen, &first, &follow)?;
                    seen.clear();
                    let mut jump = Jump::new();
                    Self::build_jump(&mut seen, &first, &mut jump);
                    Ok(Metadata { first, follow, jump })
                })
        }
    }
    #[automatically_derived]
    impl teleparse::ParseRoot for Pizza {}
};
/// Terminal symbol derived from [`MyToken`] with `terminal(Pasta = "pasta")`
#[automatically_derived]
pub struct Pasta(pub teleparse::lex::Token<MyToken>);
#[automatically_derived]
impl ::core::fmt::Debug for Pasta {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Pasta", &&self.0)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Pasta {
    #[inline]
    fn clone(&self) -> Pasta {
        let _: ::core::clone::AssertParamIsClone<teleparse::lex::Token<MyToken>>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Pasta {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Pasta {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Pasta {
    #[inline]
    fn eq(&self, other: &Pasta) -> bool {
        self.0 == other.0
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for Pasta {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<teleparse::lex::Token<MyToken>>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for Pasta {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        ::core::hash::Hash::hash(&self.0, state)
    }
}
#[automatically_derived]
impl teleparse::ToSpan for Pasta {
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
    impl ::core::convert::From<Token<MyToken>> for Pasta {
        #[inline]
        fn from(token: Token<MyToken>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl AbstractSyntaxTree for Pasta {
        type L = MyToken;
        #[inline]
        fn debug() -> Cow<'static, str> {
            Cow::Borrowed("Pasta")
        }
        #[inline]
        fn build_first(builder: &mut FirstBuilder<Self::L>) {
            let t = Self::type_id();
            let expr = FirstRel::insert_token(t, MyToken::Food, Some("pasta"));
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
            parser.parse_token_lit(MyToken::Food, "pasta", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::ParseTree for Pasta {
        type AST = Self;
        #[inline]
        fn from_ast<'s>(ast: Self, _: &mut Parser<'s, MyToken>) -> Self {
            ast
        }
    }
};
const _: () = {
    use ::std::vec::Vec;
    use ::std::result::Result;
    use ::std::collections::BTreeSet;
    use ::std::sync::OnceLock;
    use teleparse::syntax::{FirstBuilder, FollowBuilder, Jump, Metadata};
    use teleparse::GrammarError;
    #[automatically_derived]
    impl teleparse::AbstractSyntaxRoot for Pasta {
        fn metadata() -> &'static Result<Metadata<MyToken>, GrammarError> {
            static METADATA: OnceLock<Result<Metadata<MyToken>, GrammarError>> = OnceLock::new();
            METADATA
                .get_or_init(|| {
                    let mut first = FirstBuilder::new();
                    Self::build_first(&mut first);
                    let first = first.build();
                    let mut stack = Vec::new();
                    let mut seen = BTreeSet::new();
                    Self::check_left_recursive(&mut stack, &mut seen, &first)?;
                    seen.clear();
                    Self::check_first_conflict(&mut seen, &first)?;
                    seen.clear();
                    let mut follow = FollowBuilder::new(first);
                    Self::build_follow(&mut follow);
                    let (first, follow) = follow.build(<Pasta>::type_id());
                    Self::check_first_follow_conflict(&mut seen, &first, &follow)?;
                    seen.clear();
                    let mut jump = Jump::new();
                    Self::build_jump(&mut seen, &first, &mut jump);
                    Ok(Metadata { first, follow, jump })
                })
        }
    }
    #[automatically_derived]
    impl teleparse::ParseRoot for Pasta {}
};
const _: () = {
    use teleparse::Lexer as _;
    #[automatically_derived]
    impl teleparse::Lexicon for MyToken {
        type Bit = u8;
        type Lexer<'s> = teleparse::lex::LexerImpl<'s, Self>;
        type Map<T: Default + Clone> = [T; 1usize];
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
            Self::Food
        }
        fn next(&self) -> Option<Self> {
            match self {
                Self::Food => None,
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
                [teleparse::lex::Rule<MyToken>; 1usize],
            > = ::std::sync::OnceLock::new();
            let rules = RULES
                .get_or_init(|| {
                    [
                        teleparse::lex::Rule::token_literal(
                            MyToken::Food,
                            &["pizza", "pasta"],
                        ),
                    ]
                });
            Ok(teleparse::lex::LexerImpl::new(source, rules)?)
        }
    }
};
