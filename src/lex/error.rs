/// Error related to constructing lexers
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("The lexer has no rules")]
    NoRules,
    #[error("The lexer has an invalid rule: {0}")]
    InvalidRule(String),
}
