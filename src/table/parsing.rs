use std::{any::TypeId, borrow::{Borrow, BorrowMut}, collections::BTreeMap};

use derivative::Derivative;

use crate::{token::{Map, TokenSrc}, TokenType};

use super::first::FirstSet;

#[derive(Derivative)]
#[derivative(Default(new="true", bound=""))]
pub struct Parsing<T: TokenType> {
    map: BTreeMap<TypeId, ParsingEntry<T>>,
}

impl<T: TokenType> Parsing<T> {
    #[inline]
    pub fn register(&mut self, t: TypeId, first: &FirstSet<T>, id: usize)->bool{
        if self.map.contains_key(&t) {
            return false;
        }
        let mut entry = ParsingEntry::default();
        first.register_parsing(&mut entry, id);
        self.map.insert(t, entry);
        true
    }

    #[inline]
    pub fn look_up<'s>(&self, t: TypeId, token: Option<TokenSrc<'s, T>>) -> Option<usize> {
        self.map.get(&t).and_then(|entry| entry.look_up(token))
    }
}

#[derive(Derivative)]
#[derivative(Default(bound=""))]
pub struct ParsingEntry<T: TokenType> {
    /// The id to return when looking up epsilon
    epsilon: Option<usize>,
    /// when looking up a token:
    ///   - tree look up None: return the one registered for any match (or None)
    ///   - tree look up Some: return the one registered for that literal
    map: Map<T, TokenEntry>,
}

pub type TokenEntry = (Option<usize>, BTreeMap<&'static str, usize>);

impl<T: TokenType> ParsingEntry<T> {
    /// Look up the parsing table entry for a token or epsilon
    #[inline]
    pub fn look_up<'s>(&self, token: Option<TokenSrc<'s, T>>) -> Option<usize> {
        token.map(|token| self.look_up_token(&token)).unwrap_or(self.epsilon)
    }

    /// Look up the parsing table entry for a token
    #[inline]
    pub fn look_up_token(&self, token: &TokenSrc<T>) -> Option<usize> {
        let entry = self.map.get(token.ty);

        match entry.1.get(token.src).copied() {
            Some(value) => Some(value),
            None => entry.0
        }
    }

    /// Register the epsilon value
    #[inline]
    pub fn register_epsilon(&mut self, value: usize) {
        self.epsilon = Some(value);
    }

    /// Register the value for a token type
    #[inline]
    pub fn register(&mut self, value: usize, ty: T) {
        self.map.get_mut(ty).0 = Some(value);
    }

    /// Register the value for a token type + literal
    #[inline]
    pub fn register_lit(&mut self, value: usize, ty: T, lit: &'static str) {
        self.map.get_mut(ty).1.insert(lit, value);
    }
}
