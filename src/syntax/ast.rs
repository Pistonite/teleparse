use std::{any::TypeId, marker::PhantomData};
use std::borrow::Cow;
use crate::Lexicon;

use super::MetadataBuilder;

pub struct Epsilon<L: Lexicon + 'static>(PhantomData<L>);

impl<L: Lexicon + 'static> Production for Epsilon<L> {
    type L = L;
    fn debug() -> Cow<'static, str>{
        Cow::Borrowed("()")
    }
    fn register(meta: &mut MetadataBuilder<Self::L>) {
        let t = Self::id();
        if meta.visit(t, ||Self::debug().into_owned()) {
            meta.add_epsilon(t);
        }
    }
}

/// An AST node
///
/// See [module-level documentation](super) for more information.
pub trait Production: 'static {
    /// The token type of the AST node
    type L: Lexicon + 'static;

    /// Get the unique type id of the AST node,
    /// which represents one production in the grammar (multiple production in case of a
    /// union/enum)
    #[inline]
    fn id() -> TypeId {
        TypeId::of::<Self>()
    }

    /// Get the type name for the AST node for debugging
    #[inline]
    fn debug() -> Cow<'static, str>{
        Cow::Borrowed(std::any::type_name::<Self>())
    }

    fn register(meta: &mut MetadataBuilder<Self::L>);

}


#[macro_export]
#[doc(hidden)]
macro_rules! production_passthrough {
    ($P:ty) => {
        type L=<$P as $crate::syntax::Production>::L;
        #[inline]
        fn id() -> ::core::any::TypeId {
            <$P>::id()
        }
        #[inline]
        fn debug() -> ::std::borrow::Cow<'static, str> {
            <$P>::debug()
        }
        #[inline]
        fn register(meta: &mut $crate::syntax::MetadataBuilder<Self::L>) {
            <$P>::register(meta)
        }
    };
}


#[macro_export]
#[doc(hidden)]
macro_rules! register_sequence {
    ($meta:ident, $($T:ty),*) => {{
        let t = Self::id();
        if $meta.visit(t, ||Self::debug().into_owned()) {
            $meta.add_sequence(t, &[ $(<$T>::id()),* ]);
            $(<$T>::register($meta);)*
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! register_union {
    ($meta:ident, $($T:ty),*) => {{
        let t = Self::id();
        if $meta.visit(t, ||Self::debug().into_owned()) {
            $meta.add_union(t, &[ $(<$T>::id()),* ]);
            $(<$T>::register($meta);)*
        }
    }};
}
