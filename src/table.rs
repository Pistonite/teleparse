use std::borrow::{Borrow, BorrowMut, Cow};
use std::collections::btree_map::Entry;
use std::fmt::{self, Debug, Formatter};
use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::token::TokenSrc;
use crate::TokenType;

#[derive(Default, Debug, Clone)]
pub struct LitTable {
    map: BTreeMap<String, Arc<str>>,
}

impl LitTable {
    pub fn get_or_add(&mut self, lit: &str) -> Arc<str> {
        match self.map.get(lit) {
            Some(arc) => Arc::clone(arc),
            None => {
                let arc = Arc::from(lit);
                self.map.insert(lit.to_string(), Arc::clone(&arc));
                arc
            }
        }
    }
}


pub struct SyntaxTreeTable<T: TokenType> {
    map: BTreeMap<TypeId, TermSet<T>>,
}

impl<T: TokenType> Default for SyntaxTreeTable<T> {
    fn default() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl<T: TokenType> SyntaxTreeTable<T> {
    /// Return if the token can follow the given SyntaxTree type
    #[inline]
    pub fn can_follow<'s>(&self, st: TypeId, token: Option<TokenSrc<'s, T>>) -> bool {
        self.map.get(&st).map(|set| set.contains(token)).unwrap_or(false)
    }

    #[inline]
    pub fn get_mut(&mut self, st: TypeId)-> &mut TermSet<T> {
        self.map.entry(st).or_default()
    }

    #[inline]
    pub fn get(&self, st: TypeId)-> Cow<'_, TermSet<T>> {
        match self.map.get(&st) {
            Some(set) => Cow::Borrowed(set),
            None => Cow::Owned(TermSet::default())
        }
    }

    #[inline]
    pub fn init<F: FnOnce(&mut Self) -> (TermSet<T>, bool)>(&mut self, st: TypeId, f: F) -> bool{
        if self.map.contains_key(&st) {
            return true;
        }
        let (set, is_ll1) = f(self);
        self.map.insert(st, set);
        is_ll1
    }
}

#[derive(Clone)]
pub struct TermSet<T: TokenType> {
    contains_eof: bool,
    array: T::Follow,
}

impl<T: TokenType> Debug for TermSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("TermSet");
        s.field("eof", &self.contains_eof);
        let slice: &[LitSet] = self.array.borrow();
        for (ty, set) in slice.iter().enumerate() {
            s.field(&ty.to_string(), set);
        }
        s.finish()
    }
}

impl<T: TokenType> Default for TermSet<T> {
    fn default() -> Self {
        Self {
            contains_eof: false,
            array: T::Follow::default(),
        }
    }
}

impl<T: TokenType> TermSet<T> {
    pub fn contains<'s>(&self, token: Option<TokenSrc<'s, T>>) -> bool {
        match token {
            None => self.contains_eof,
            Some(token) => {
                self.get(token.ty).contains(&token.src)
            }
        }
    }

    #[inline]
    pub fn insert_eof(&mut self) {
        self.contains_eof = true;
    }

    #[inline]
    pub fn contains_eof(&self) -> bool {
        self.contains_eof
    }

    #[inline]
    pub fn remove_eof(&mut self) {
        self.contains_eof = false;
    }

    #[inline]
    pub fn insert_token_type(&mut self, ty: T) {
        self.get_mut(ty).insert_any();
    }

    #[inline]
    pub fn insert_token_type_match(&mut self, ty: T, lit: Arc<str>) {
        self.get_mut(ty).insert(lit);
    }

    pub fn union(&mut self, other: &Self) {
        self.contains_eof |= other.contains_eof;
        for (set, other_set) in self.array.borrow_mut().iter_mut().zip(other.array.borrow().iter()) {
            set.union(other_set);
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        if self.contains_eof && other.contains_eof {
            return true;
        }
        for (set, other_set) in self.array.borrow().iter().zip(other.array.borrow().iter()) {
            if set.intersects(other_set) {
                return true;
            }
        }
        return false;
    }

    #[inline]
    fn get(&self, ty: T) -> &LitSet {
        &self.array.borrow()[ty.id()]
    }

    #[inline]
    fn get_mut(&mut self, ty: T) -> &mut LitSet {
        &mut self.array.borrow_mut()[ty.id()]
    }
}

#[derive(Clone)]
pub enum LitSet {
    Match(BTreeSet<Arc<str>>),
    Any,
}

impl Debug for LitSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Match(set) => write!(f, "{:?}", set),
            Self::Any => write!(f, "*"),
        }
    }
}

impl Default for LitSet {
    fn default() -> Self {
        Self::Match(BTreeSet::new())
    }
}

impl LitSet {
    #[inline]
    pub fn insert_any(&mut self) {
        *self = Self::Any;
    }
    #[inline]
    pub fn insert(&mut self, lit: Arc<str>) {
        if let Self::Match(set) = self {
            set.insert(lit);
        }
    }
    #[inline]
    pub fn contains(&self, lit: &str) -> bool {
        match self {
            Self::Match(set) => set.contains(lit),
            Self::Any => true,
        }
    }

    pub fn union(&mut self, other: &Self) {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                set.extend(other_set.iter().cloned());
            }
            (s, _) => {
                *s= Self::Any;
            }
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                set.iter().any(|lit| {
                    let lit: &str = lit;
                    other_set.contains(lit)
                })
            }
            (Self::Any, _) | (_, Self::Any) => true,
        }
    }

}
