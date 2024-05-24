pub fn add(left: usize, right: usize) -> usize {
    println!("{:#?}", Test { _debug_me: true });
    left + right
}

#[llnparse_macros::test_my_macro]
struct Test {
    _debug_me: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
