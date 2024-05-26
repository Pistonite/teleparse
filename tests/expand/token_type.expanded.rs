use llnparse::llnparse_derive;
#[repr(u16)]
pub enum TokenU16 {
    X0000 = 0x1u16,
    X0001 = 0x2u16,
    X0010 = 0x4u16,
    X0011 = 0x8u16,
    X0100 = 0x10u16,
    X0101 = 0x20u16,
    X0110 = 0x40u16,
    X0111 = 0x80u16,
    X1000 = 0x100u16,
    X1001 = 0x200u16,
    X1010 = 0x400u16,
    X1011 = 0x800u16,
    X1100 = 0x1000u16,
    X1101 = 0x2000u16,
    X1110 = 0x4000u16,
    X1111 = 0x8000u16,
}
#[automatically_derived]
impl ::core::fmt::Debug for TokenU16 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                TokenU16::X0000 => "X0000",
                TokenU16::X0001 => "X0001",
                TokenU16::X0010 => "X0010",
                TokenU16::X0011 => "X0011",
                TokenU16::X0100 => "X0100",
                TokenU16::X0101 => "X0101",
                TokenU16::X0110 => "X0110",
                TokenU16::X0111 => "X0111",
                TokenU16::X1000 => "X1000",
                TokenU16::X1001 => "X1001",
                TokenU16::X1010 => "X1010",
                TokenU16::X1011 => "X1011",
                TokenU16::X1100 => "X1100",
                TokenU16::X1101 => "X1101",
                TokenU16::X1110 => "X1110",
                TokenU16::X1111 => "X1111",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for TokenU16 {
    #[inline]
    fn clone(&self) -> TokenU16 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for TokenU16 {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TokenU16 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TokenU16 {
    #[inline]
    fn eq(&self, other: &TokenU16) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TokenU16 {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for TokenU16 {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl llnparse::TokenType for TokenU16 {
    type Repr = u16;
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
        Self::X0000
    }
    fn next(&self) -> Option<Self> {
        match self {
            Self::X1111 => None,
            _ => {
                let repr = self.to_repr();
                let next = repr << 1;
                Some(unsafe { std::mem::transmute(next) })
            }
        }
    }
}
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
#[automatically_derived]
impl llnparse::TokenType for TokenU32 {
    type Repr = u32;
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
}
