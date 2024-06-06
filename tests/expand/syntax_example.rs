use teleparse::prelude::*;

#[derive_syntax(TokenType)]
pub enum UnionTest {
    Foo,
    Bar,
    Biz
}

#[derive_syntax(TokenType)]
pub enum UnionTestOverride {
    Foo,
    Bar(Quaak),
    Biz(Box<Biz>)
}
