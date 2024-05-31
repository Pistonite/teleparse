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

    /// Union B into A (A = A U B). Return if A is changed
    pub fn union(&mut self, a: TypeId, b: TypeId) -> bool {
        if a == b {
            return false;
        }
        let b_set = match self.map.remove(&b) {
            Some(b) => b,
            None => return false,
        };
        let changed = self.get_mut(a).union(&b_set);
        self.map.insert(b, b_set);
        changed
    }

    /// Union (B - e) into A (A = A U (B - e)). Return if A is changed
    pub fn union_skip_empty(&mut self, a: TypeId, b: TypeId) -> bool {
        if a == b {
            return false;
        }
        let b_set = match self.map.remove(&b) {
            Some(b) => b,
            None => return false,
        };
        let changed = self.get_mut(a).union_skip_empty(&b_set);
        self.map.insert(b, b_set);
        changed
    }

    // /// Initialize the table with a new AST type
    // /// 
    // #[inline]
    // pub fn init<Pre: FnOnce(&mut Self) -> (), Post: FnOnce(&mut TermSet<T>) -> ()>(
    //     &mut self, st: TypeId, pre: Pre, post: Post) {
    //     // insert the key
    //     match self.map.entry(st) {
    //         Entry::Occupied(_) => return,
    //         Entry::Vacant(e) => e.insert(TermSet::default())
    //     };
    //     pre(self);
    //     post(self.get_mut(st));
    // }
}

#[derive(Clone)]
pub struct TermSet<T: TokenType> {
    contains_empty: bool,
    array: T::Follow,
}

impl<T: TokenType> Debug for TermSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("TermSet");
        s.field("eof", &self.contains_empty);
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
            contains_empty: false,
            array: T::Follow::default(),
        }
    }
}

impl<T: TokenType> TermSet<T> {
    pub fn contains<'s>(&self, token: Option<TokenSrc<'s, T>>) -> bool {
        match token {
            None => self.contains_empty,
            Some(token) => {
                self.get(token.ty).contains(&token.src)
            }
        }
    }

    #[inline]
    pub fn insert_empty(&mut self) {
        self.contains_empty = true;
    }

    #[inline]
    pub fn contains_empty(&self) -> bool {
        self.contains_empty
    }

    #[inline]
    pub fn remove_empty(&mut self) {
        self.contains_empty = false;
    }

    #[inline]
    pub fn insert_token_type(&mut self, ty: T) {
        self.get_mut(ty).insert_any();
    }

    #[inline]
    pub fn insert_token_type_match(&mut self, ty: T, lit: Arc<str>) {
        self.get_mut(ty).insert(lit);
    }

    /// Union other into self (self = self U other). Return if self is changed
    #[inline]
    pub fn union(&mut self, other: &Self) -> bool {
        self.contains_empty |= other.contains_empty;
        self.union_skip_empty(other)
    }

    /// Union (other - e) into self (self = self U (other - e)). Return if self is changed
    pub fn union_skip_empty(&mut self, other: &Self) -> bool {
        let mut changed = false;
        for (set, other_set) in self.array.borrow_mut().iter_mut().zip(other.array.borrow().iter()) {
            let next_changed = set.union(other_set);
            changed |= next_changed;
        }
        changed
    }

    pub fn intersects(&self, other: &Self) -> bool {
        if self.contains_empty && other.contains_empty {
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

/// Set of literals
///
/// Since the set of literals needed for a [`TokenType`]
/// is finite, this uses `Arc<str>` internally to efficiently union with other sets.
#[derive(Clone)]
pub enum LitSet {
    /// A finite set
    Match(BTreeSet<Arc<str>>),
    /// Universal set
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

    /// Union other into self (self = self U other). Return if self is changed
    pub fn union(&mut self, other: &Self) -> bool {
        match (self, other) {
            (Self::Match(set), Self::Match(other_set)) => {
                let old_size = set.len();
                set.extend(other_set.iter().cloned());
                old_size != set.len()
            }
            (s, _) => {
                let is_self_any = matches!(s, Self::Any);
                *s= Self::Any;
                !is_self_any
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
