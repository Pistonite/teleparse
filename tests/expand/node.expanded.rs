use teleparse::teleparse_derive;
#[teleparse_derive(Node)]
pub struct Quote<S: From<String>, ST: SyntaxTree>(Node<S>, PhantomData<ST>);
#[teleparse_derive(Node)]
pub struct Struct {
    pub field: Node<String>,
    pub field2: i32,
}
