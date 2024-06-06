#[test]
fn macro_expand() {
    macrotest::expand("tests/expand/*.rs");
}

mod own_src {
    #[test]
    fn tuple() {
        macrotest::expand("src/tp/tuple.rs");
    }
}


