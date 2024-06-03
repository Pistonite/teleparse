//! # Predictive parsing table
//!
//! The `jump` module implements a predictive parsing table for productions in the form:
//! ```text
//! X -> Y1 | Y2 | ... | Yn
//! ```
//!
//! If the grammar is LL(1), then there is no FIRST/FIRST conflict (see [FIRST set](super::first)).
//! This means there is a function `F` such that:
//! ```text
//! F(X, ty, lit) => Yi
//! where
//!   X is the symbol being produced
//!   ty is the type of the next token
//!   lit is the content of the next token, if any
//! ```
//!
//! [`Jump`] implements such function and uses [`TypeId`] to identify AST types.
//! The output of the function is a `usize` id indicating which path to take.
//! The AST types handles populating the table and mapping between the id and which path to take.
//!
//! Like FIRST/FOLLOW, this table is also lazy-computed once and stored in a static reference using
//! [`OnceLock`](std::sync::OnceLock). The lock is only queried once at the first call to the
//! parser.

use std::any::TypeId;
use std::collections::BTreeMap;

use derivative::Derivative;

use crate::lex::{Map, TokenSrc};
use crate::Lexicon;

use super::FirstSet;

/// Implementation of the predictive parsing table
///
/// See [module-level documentation](self) for more information.
#[derive(Derivative)]
#[derivative(Default(new="true", bound=""))]
pub struct Jump<L: Lexicon> {
    /// This maps (X) -> ((ty, lit) -> Yi)
    map: BTreeMap<TypeId, JumpTable<L>>,
}

impl<L: Lexicon> Jump<L> {
    #[inline]
    pub fn register(&mut self, t: TypeId, first: &FirstSet<L>, id: usize)->bool{
        if self.map.contains_key(&t) {
            return false;
        }
        let mut entry = JumpTable::default();
        first.add_to_jump_table(&mut entry, id);
        self.map.insert(t, entry);
        true
    }

    #[inline]
    pub fn look_up<'s>(&self, t: TypeId, token: Option<TokenSrc<'s, L>>) -> Option<usize> {
        self.map.get(&t).and_then(|entry| entry.look_up(token))
    }
}

/// The parse table entry for a specific AST type
///
/// See [module-level documentation](self) for more information.
///
/// This maps (ty, lit) -> Yi
#[derive(Derivative)]
#[derivative(Default(bound=""))]
pub struct JumpTable<L: Lexicon> {
    /// The id to return when looking up epsilon
    epsilon: Option<usize>,
    /// when looking up a token:
    ///   - tree look up None: return the one registered for any match (or None)
    ///   - tree look up Some: return the one registered for that literal
    map: Map<L, LitJumpTable>,
}

pub type LitJumpTable = (Option<usize>, BTreeMap<&'static str, usize>);

impl<L: Lexicon> JumpTable<L> {
    /// Look up the parsing table entry for a token or epsilon
    #[inline]
    pub fn look_up<'s>(&self, token: Option<TokenSrc<'s, L>>) -> Option<usize> {
        token.map(|token| self.look_up_token(&token)).unwrap_or(self.epsilon)
    }

    /// Look up the parsing table entry for a token
    #[inline]
    pub fn look_up_token(&self, token: &TokenSrc<L>) -> Option<usize> {
        let entry = self.map.get(token.ty);

        match entry.1.get(token.src).copied() {
            Some(value) => Some(value),
            None => entry.0
        }
    }

    /// Register `epsilon -> value`
    #[inline]
    pub fn register_epsilon(&mut self, value: usize) {
        self.epsilon = Some(value);
    }

    /// Register `(ty, *) -> value`
    #[inline]
    pub fn register(&mut self, value: usize, ty: L) {
        self.map.get_mut(ty).0 = Some(value);
    }

    /// Register `(ty, lit) -> value`
    #[inline]
    pub fn register_lit(&mut self, value: usize, ty: L, lit: &'static str) {
        self.map.get_mut(ty).1.insert(lit, value);
    }
}
