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
pub struct Pizza(pub teleparse::Token<MyToken>);
#[automatically_derived]
impl ::core::clone::Clone for Pizza {
    #[inline]
    fn clone(&self) -> Pizza {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<MyToken>>;
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
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<MyToken>>;
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
    fn lo(&self) -> teleparse::Pos {
        use teleparse::ToSpan;
        self.0.lo()
    }
    fn hi(&self) -> teleparse::Pos {
        use teleparse::ToSpan;
        self.0.hi()
    }
}
impl Pizza {
    pub fn from_span<S: ::core::convert::Into<teleparse::Span>>(span: S) -> Self {
        Self::from(teleparse::Token::new(span, MyToken::Food))
    }
}
const _: () = {
    #[automatically_derived]
    impl ::core::convert::From<teleparse::Token<MyToken>> for Pizza {
        fn from(token: teleparse::Token<MyToken>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Pizza {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }
    #[automatically_derived]
    impl teleparse::syntax::Production for Pizza {
        type L = MyToken;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("pizza")
        }
        fn register(meta: &mut teleparse::syntax::MetadataBuilder<Self::L>) {
            let t = <Self as teleparse::syntax::Production>::id();
            if meta.visit(t, || Self::debug().into_owned()) {
                meta.add_terminal(t, MyToken::Food, Some("pizza"));
            }
        }
    }
    #[automatically_derived]
    impl teleparse::parser::Produce for Pizza {
        type Prod = Self;
        fn produce(
            parser: &mut teleparse::Parser<
                '_,
                <Self::Prod as teleparse::syntax::Production>::L,
            >,
            meta: &teleparse::syntax::Metadata<
                <Self::Prod as teleparse::syntax::Production>::L,
            >,
        ) -> teleparse::syntax::Result<
            Self,
            <Self::Prod as teleparse::syntax::Production>::L,
        > {
            let follow = meta.follow.get(&<Self as teleparse::syntax::Production>::id());
            parser.parse_token_lit(MyToken::Food, "pizza", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::parser::Root for Pizza {
        fn metadata() -> &'static ::core::result::Result<
            teleparse::syntax::Metadata<
                <Self::Prod as teleparse::syntax::Production>::L,
            >,
            teleparse::GrammarError,
        > {
            use teleparse::syntax::Production;
            static METADATA: ::std::sync::OnceLock<
                ::core::result::Result<
                    teleparse::syntax::Metadata<
                        <<Pizza as teleparse::parser::Produce>::Prod as teleparse::syntax::Production>::L,
                    >,
                    teleparse::GrammarError,
                >,
            > = ::std::sync::OnceLock::new();
            METADATA
                .get_or_init(|| {
                    teleparse::syntax::Metadata::build_for::<
                        <Pizza as teleparse::parser::Produce>::Prod,
                    >()
                })
        }
    }
};
/// Terminal symbol derived from [`MyToken`] with `terminal(Pasta = "pasta")`
pub struct Pasta(pub teleparse::Token<MyToken>);
#[automatically_derived]
impl ::core::clone::Clone for Pasta {
    #[inline]
    fn clone(&self) -> Pasta {
        let _: ::core::clone::AssertParamIsClone<teleparse::Token<MyToken>>;
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
        let _: ::core::cmp::AssertParamIsEq<teleparse::Token<MyToken>>;
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
    fn lo(&self) -> teleparse::Pos {
        use teleparse::ToSpan;
        self.0.lo()
    }
    fn hi(&self) -> teleparse::Pos {
        use teleparse::ToSpan;
        self.0.hi()
    }
}
impl Pasta {
    pub fn from_span<S: ::core::convert::Into<teleparse::Span>>(span: S) -> Self {
        Self::from(teleparse::Token::new(span, MyToken::Food))
    }
}
const _: () = {
    #[automatically_derived]
    impl ::core::convert::From<teleparse::Token<MyToken>> for Pasta {
        fn from(token: teleparse::Token<MyToken>) -> Self {
            Self(token)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Pasta {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            self.0.fmt(f)
        }
    }
    #[automatically_derived]
    impl teleparse::syntax::Production for Pasta {
        type L = MyToken;
        fn debug() -> ::std::borrow::Cow<'static, str> {
            ::std::borrow::Cow::Borrowed("pasta")
        }
        fn register(meta: &mut teleparse::syntax::MetadataBuilder<Self::L>) {
            let t = <Self as teleparse::syntax::Production>::id();
            if meta.visit(t, || Self::debug().into_owned()) {
                meta.add_terminal(t, MyToken::Food, Some("pasta"));
            }
        }
    }
    #[automatically_derived]
    impl teleparse::parser::Produce for Pasta {
        type Prod = Self;
        fn produce(
            parser: &mut teleparse::Parser<
                '_,
                <Self::Prod as teleparse::syntax::Production>::L,
            >,
            meta: &teleparse::syntax::Metadata<
                <Self::Prod as teleparse::syntax::Production>::L,
            >,
        ) -> teleparse::syntax::Result<
            Self,
            <Self::Prod as teleparse::syntax::Production>::L,
        > {
            let follow = meta.follow.get(&<Self as teleparse::syntax::Production>::id());
            parser.parse_token_lit(MyToken::Food, "pasta", follow).map(Self::from)
        }
    }
    #[automatically_derived]
    impl teleparse::parser::Root for Pasta {
        fn metadata() -> &'static ::core::result::Result<
            teleparse::syntax::Metadata<
                <Self::Prod as teleparse::syntax::Production>::L,
            >,
            teleparse::GrammarError,
        > {
            use teleparse::syntax::Production;
            static METADATA: ::std::sync::OnceLock<
                ::core::result::Result<
                    teleparse::syntax::Metadata<
                        <<Pasta as teleparse::parser::Produce>::Prod as teleparse::syntax::Production>::L,
                    >,
                    teleparse::GrammarError,
                >,
            > = ::std::sync::OnceLock::new();
            METADATA
                .get_or_init(|| {
                    teleparse::syntax::Metadata::build_for::<
                        <Pasta as teleparse::parser::Produce>::Prod,
                    >()
                })
        }
    }
};
const _: () = {
    pub enum DerivedLogos {
        #[token("pizza")]
        #[token("pasta")]
        Food,
    }
    impl<'s> ::logos::Logos<'s> for DerivedLogos {
        type Error = ();
        type Extras = ();
        type Source = str;
        fn lex(lex: &mut ::logos::Lexer<'s, Self>) {
            use ::logos::internal::{LexerInternal, CallbackResult};
            type Lexer<'s> = ::logos::Lexer<'s, DerivedLogos>;
            fn _end<'s>(lex: &mut Lexer<'s>) {
                lex.end()
            }
            fn _error<'s>(lex: &mut Lexer<'s>) {
                lex.bump_unchecked(1);
                lex.error();
            }
            #[inline]
            fn goto2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Food));
            }
            #[inline]
            fn goto7_at2_with5<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 3usize]>(2usize) {
                    Some(b"sta") => {
                        lex.bump_unchecked(5usize);
                        goto2_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Food));
            }
            #[inline]
            fn goto6_at2_with5<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 3usize]>(2usize) {
                    Some(b"zza") => {
                        lex.bump_unchecked(5usize);
                        goto1_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto5_at1_with5<'s>(lex: &mut Lexer<'s>) {
                let byte = unsafe { lex.read_unchecked::<u8>(1usize) };
                match byte {
                    b'a' => goto7_at2_with5(lex),
                    b'i' => goto6_at2_with5(lex),
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto8<'s>(lex: &mut Lexer<'s>) {
                let arr = match lex.read::<&[u8; 5usize]>() {
                    Some(arr) => arr,
                    None => return _end(lex),
                };
                match arr[0] {
                    b'p' => goto5_at1_with5(lex),
                    _ => _error(lex),
                }
            }
            goto8(lex)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<DerivedLogos> for MyToken {
        fn from(token: DerivedLogos) -> Self {
            match token {
                DerivedLogos::Food => Self::Food,
            }
        }
    }
    #[automatically_derived]
    impl teleparse::Lexicon for MyToken {
        type Bit = u8;
        type Lexer<'s> = teleparse::lex::LogosLexerWrapper<'s, Self, DerivedLogos>;
        type Map<T: Default + Clone> = [T; 1usize];
        fn id(&self) -> usize {
            *self as usize
        }
        fn from_id_unchecked(id: usize) -> Self {
            unsafe { std::mem::transmute(id) }
        }
        fn to_bit(&self) -> Self::Bit {
            (1 << self.id()) as Self::Bit
        }
        fn first() -> Self {
            Self::Food
        }
        fn next(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Food => None,
                _ => {
                    let next = self.id() + 1;
                    Some(Self::from_id_unchecked(next))
                }
            }
        }
        fn should_extract(&self) -> bool {
            match self {
                _ => false,
            }
        }
        fn lexer(
            source: &str,
        ) -> ::core::result::Result<Self::Lexer<'_>, teleparse::GrammarError> {
            use teleparse::__priv::logos::Logos;
            Ok(teleparse::lex::LogosLexerWrapper::new(DerivedLogos::lexer(source)))
        }
    }
};
fn main() {}
