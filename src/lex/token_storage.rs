use deref_derive::{Deref, DerefMut};

use super::{Lexicon, Token};


/// One cell in a [`TokenStorage`], with the token and any semantic types added
#[derive(Deref, DerefMut)]
pub struct TokenCell<L: Lexicon> {
    #[deref]
    token: Token<L>,
    // pub ty_set: TokenTySet<T>,
}

impl<L: Lexicon> From<Token<L>> for TokenCell<L> {
    fn from(token: Token<L>) -> Self {
        Self {
            token,
            // ty_set: TokenTySet::new(),
        }
    }
}

/// Stores token and semantic information during parsing
pub struct TokenStorage<L: Lexicon> {
    tokens: Vec<TokenCell<L>>,
}

impl<L: Lexicon> TokenStorage<L> {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn at(&self, pos: usize) -> Option<&TokenCell<L>> {
        self.tokens.get(pos)
    }

    /// Add the token to the end of the storage.
    ///
    /// # panic
    /// Will panic if the token is not strictly after the last token
    pub fn add_last(&mut self, token: Token<L>) {
        if let Some(last) = self.tokens.last() {
            if token.span.lo < last.token.span.hi {
                panic!("new token is not strictly after the last token");
            }
        }
        self.tokens.push(token.into());
    }
}


