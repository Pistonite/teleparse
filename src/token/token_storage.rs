
use std::ops::{Index, IndexMut};

use deref_derive::{Deref, DerefMut};

use super::{TokenTySet, TokenType, Token};


/// One cell in the token storage, with the token and any semantic types added
#[derive(Deref, DerefMut)]
pub struct TokenCell<T: TokenType> {
    #[deref]
    token: Token<T>,
    pub ty_set: TokenTySet<T>,
}

impl<T: TokenType> From<Token<T>> for TokenCell<T> {
    fn from(token: Token<T>) -> Self {
        Self {
            token,
            ty_set: TokenTySet::new(),
        }
    }
}

pub struct TokenStorage<T: TokenType> {
    tokens: Vec<TokenCell<T>>,
}

impl<T: TokenType> TokenStorage<T> {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn at(&self, pos: usize) -> Option<&TokenCell<T>> {
        self.tokens.get(pos)
    }

    /// Add the token to the end of the storage.
    ///
    /// # panic
    /// Will panic if the token is not strictly after the last token
    pub fn add_last(&mut self, token: Token<T>) {
        if let Some(last) = self.tokens.last() {
            if token.span.lo < last.token.span.hi {
                panic!("new token is not strictly after the last token");
            }
        }
        self.tokens.push(token.into());
    }
}


