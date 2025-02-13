use teleparse::prelude::*;
pub enum UnionTest {
    Foo,
    Bar,
    Biz,
}
pub enum UnionTestOverride {
    Foo,
    Bar(Quaak),
    Biz(Box<Biz>),
}
