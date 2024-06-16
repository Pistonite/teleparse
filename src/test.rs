//! Test utilities

use crate::derive_lexicon;

#[derive_lexicon]
pub enum TestTokenType {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive_lexicon]
#[teleparse(
    ignore(r#"\s+"#), // ignore whitespaces, separate multiple with comma
)]
pub enum MathTokenType {
    #[teleparse(regex(r#"[a-zA-Z]+"#), terminal(Ident))]
    Ident,
    #[teleparse(terminal(
        OpAdd = "+",
        OpMul = "*",
    ))]
    Op,
    /// Parentheses
    #[teleparse(terminal(
        ParamOpen = "(",
        ParamClose = ")"
    ))]
    Param,

    #[teleparse(regex(r"\d+"), terminal(Integer))]
    Integer
}

pub mod prelude {
    macro_rules! assert_not_ll1 {
        ($pt:ty, $err:expr) => {
            use $crate::Root;
            let err = if let Err(e) = <$pt as Root>::metadata() {
                e.clone()
            } else {
                panic!("Expected {} to be not LL(1), but it is", stringify!($pt));
            };
            assert_eq!(err, $err);
            assert!(<$pt>::parse("").is_err());
        }
    }
    pub(crate) use assert_not_ll1;

}
