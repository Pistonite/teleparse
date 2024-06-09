use teleparse::prelude::*;
#[repr(usize)]
pub enum MyToken {
    Comment = 0usize,
}
#[automatically_derived]
impl ::core::fmt::Debug for MyToken {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Comment")
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
const _: () = {
    pub enum DerivedLogos {
        #[regex(r"/\*([^\*]|(\*[^/]))*\*/")]
        Comment,
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
            fn goto1_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Comment));
            }
            #[inline]
            fn goto2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"*/") => {
                        lex.bump_unchecked(2usize);
                        goto1_x(lex)
                    }
                    _ => lex.error(),
                }
            }
            #[inline]
            fn goto1_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                lex.set(Ok(DerivedLogos::Comment));
            }
            #[inline]
            fn goto2_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                match lex.read::<&[u8; 2usize]>() {
                    Some(b"*/") => {
                        lex.bump_unchecked(2usize);
                        goto1_ctx2_x(lex)
                    }
                    _ => goto2_x(lex),
                }
            }
            #[inline]
            fn pattern0(byte: u8) -> bool {
                match byte {
                    0u8..=b'.' | b'0'..=255u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto5_at1_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read_at::<u8>(1usize) {
                    Some(byte) => byte,
                    None => return goto2_x(lex),
                };
                match byte {
                    byte if pattern0(byte) => {
                        lex.bump_unchecked(2usize);
                        goto3_ctx2_x(lex)
                    }
                    _ => goto2_x(lex),
                }
            }
            #[inline]
            fn pattern1(byte: u8) -> bool {
                match byte {
                    0u8..=b')' | b'+'..=255u8 => true,
                    _ => false,
                }
            }
            #[inline]
            fn goto3_ctx2_x<'s>(lex: &mut Lexer<'s>) {
                let byte = match lex.read::<u8>() {
                    Some(byte) => byte,
                    None => return goto2_ctx2_x(lex),
                };
                match byte {
                    b'*' => goto5_at1_ctx2_x(lex),
                    byte if pattern1(byte) => {
                        lex.bump_unchecked(1usize);
                        goto3_ctx2_x(lex)
                    }
                    _ => goto2_ctx2_x(lex),
                }
            }
            #[inline]
            fn goto8_at1_with2<'s>(lex: &mut Lexer<'s>) {
                match lex.read_at::<&[u8; 1usize]>(1usize) {
                    Some(b"*") => {
                        lex.bump_unchecked(2usize);
                        goto3_ctx2_x(lex)
                    }
                    _ => _error(lex),
                }
            }
            #[inline]
            fn goto9<'s>(lex: &mut Lexer<'s>) {
                let arr = match lex.read::<&[u8; 2usize]>() {
                    Some(arr) => arr,
                    None => return _end(lex),
                };
                match arr[0] {
                    b'/' => goto8_at1_with2(lex),
                    _ => _error(lex),
                }
            }
            goto9(lex)
        }
    }
    #[automatically_derived]
    impl ::core::convert::From<DerivedLogos> for MyToken {
        fn from(token: DerivedLogos) -> Self {
            match token {
                DerivedLogos::Comment => Self::Comment,
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
        fn from_id(id: usize) -> Self {
            unsafe { std::mem::transmute(id) }
        }
        fn to_bit(&self) -> Self::Bit {
            (1 << self.id()) as Self::Bit
        }
        fn first() -> Self {
            Self::Comment
        }
        fn next(&self) -> ::core::option::Option<Self> {
            match self {
                Self::Comment => None,
                _ => {
                    let next = self.id() + 1;
                    Some(Self::from_id(next))
                }
            }
        }
        fn should_extract(&self) -> bool {
            match self {
                Self::Comment => true,
                _ => false,
            }
        }
        fn lexer<'s>(
            source: &'s str,
        ) -> ::core::result::Result<Self::Lexer<'s>, teleparse::GrammarError> {
            use teleparse::__priv::logos::Logos;
            Ok(teleparse::lex::LogosLexerWrapper::new(DerivedLogos::lexer(source)))
        }
    }
};
fn main() {
    let input = "/* This is a comment */";
    let mut lexer = MyToken::lexer(input).unwrap();
    match (&lexer.next(), &(None, Some(Token::new(0..23, MyToken::Comment)))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    if !MyToken::Comment.should_extract() {
        ::core::panicking::panic("assertion failed: MyToken::Comment.should_extract()")
    }
}
