use llnparse::llnparse_derive;
#[repr(u8)]
pub enum MyTokenType {
    Comment = 0x1u8,
    Keyword = 0x2u8,
    Ident = 0x4u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for MyTokenType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                MyTokenType::Comment => "Comment",
                MyTokenType::Keyword => "Keyword",
                MyTokenType::Ident => "Ident",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MyTokenType {
    #[inline]
    fn clone(&self) -> MyTokenType {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for MyTokenType {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for MyTokenType {}
#[automatically_derived]
impl ::core::cmp::PartialEq for MyTokenType {
    #[inline]
    fn eq(&self, other: &MyTokenType) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for MyTokenType {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for MyTokenType {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl llnparse::TokenType for MyTokenType {
    type Repr = u8;
    #[inline]
    fn should_extract(&self) -> bool {
        match self {
            Self::Comment => true,
            _ => false,
        }
    }
    #[inline]
    fn to_repr(&self) -> Self::Repr {
        *self as Self::Repr
    }
    #[inline]
    fn first() -> Self {
        Self::Comment
    }
    fn next(&self) -> Option<Self> {
        match self {
            Self::Ident => None,
            _ => {
                let repr = self.to_repr();
                let next = repr << 1;
                Some(unsafe { std::mem::transmute(next) })
            }
        }
    }
}
