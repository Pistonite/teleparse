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
            Self::Comment
        }
        fn next(&self) -> Option<Self> {
            match self {
                Self::Comment => None,
                _ => {
                    let next = self.id() + 1;
                    Some(Self::from_id(next))
                }
            }
        }
        #[inline]
        fn should_extract(&self) -> bool {
            match self {
                Self::Comment => true,
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
                    [teleparse::lex::Rule::token(MyToken::Comment, r"^/\*[\s\S]*?\*/")]
                });
            Ok(teleparse::lex::LexerImpl::new(source, rules)?)
        }
    }
};
