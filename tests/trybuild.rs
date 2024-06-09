#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
    t.pass("tests/expand/pizza.rs");
    t.pass("tests/expand/comment.rs");
}
