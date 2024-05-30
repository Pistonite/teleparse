use teleparse::teleparse_derive;
pub struct Quote<S: From<String>, ST: SyntaxTree>(Node<S>, PhantomData<ST>);
#[automatically_derived]
impl<S: From<String>, ST: SyntaxTree> teleparse::ToSpan for Quote<S, ST> {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.0.span()
    }
}
#[automatically_derived]
impl<S: From<String>, ST: SyntaxTree> ::core::convert::From<teleparse::tp::Node<S>>
for Quote<S, ST> {
    #[inline]
    fn from(node: teleparse::tp::Node<S>) -> Self {
        Self(node, ::core::default::Default::default())
    }
}
#[automatically_derived]
impl<S: From<String>, ST: SyntaxTree> ::core::ops::Deref for Quote<S, ST> {
    type Target = S;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}
#[automatically_derived]
impl<S: From<String>, ST: SyntaxTree> ::core::ops::DerefMut for Quote<S, ST> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.value
    }
}
pub struct Struct {
    pub field: Node<String>,
    pub field2: i32,
}
#[automatically_derived]
impl teleparse::ToSpan for Struct {
    #[inline]
    fn span(&self) -> teleparse::Span {
        self.field.span()
    }
}
#[automatically_derived]
impl ::core::convert::From<teleparse::tp::Node<String>> for Struct {
    #[inline]
    fn from(node: teleparse::tp::Node<String>) -> Self {
        Self { field }
    }
}
#[automatically_derived]
impl ::core::ops::Deref for Struct {
    type Target = String;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.field.value
    }
}
#[automatically_derived]
impl ::core::ops::DerefMut for Struct {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field.value
    }
}
