use deref_derive::{Deref, DerefMut};

use crate::{Span, ToSpan};

#[derive(Deref, DerefMut, ToSpan, Clone, PartialEq)]
pub struct Node<T> {
    pub span: Span,
    #[deref]
    pub value: T,
}
pub use teleparse_macros::Node;

impl<T> Node<T> {
    pub fn new<S: Into<Span>>(span: S, value: T) -> Self {
        Self { span: span.into(), value }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} => ", self.span)?;
        self.value.fmt(f)
    }
}

