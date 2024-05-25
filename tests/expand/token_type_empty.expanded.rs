use llnparse::llnparse_derive;
pub enum MyTokenType {}
#[automatically_derived]
impl llnparse::TokenType for MyTokenType {
    fn should_extract(&self) -> bool {
        match self {
            _ => false,
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for MyTokenType {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {}
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
        match *self {}
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
        match *self {}
    }
}
