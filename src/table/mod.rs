use std::borrow::{Borrow, BorrowMut, Cow};
use std::collections::btree_map::Entry;
use std::fmt::{self, Debug, Formatter};
use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::token::TokenSrc;
use crate::TokenType;

pub mod first;
pub mod follow;

mod lit_set;
pub use lit_set::LitSet;
// mod prod_set;

mod term_set;
// pub use term_set::{FollowSet};
mod util;

// #[derive(Debug, Default)]
// pub struct First<T: TokenType>(Table<T>, FirstSet<T>);
// impl<T: TokenType> First<T> {
//     /// Get FIRST(T) for insertion
//     #[inline]
//     pub fn get_mut(&mut self, t: TypeId) -> &mut FirstSet<T> {
//         self.0.get_mut(t).as_first_mut()
//     }
//
//     /// Get FIRST(T)
//     pub fn get(&self, t: TypeId) -> &FirstSet<T> {
//         match self.0.map.get(&t) {
//             Some(set) => set.as_first(),
//             None => &self.1
//         }
//     }
//
//     /// Union FIRST(B) into FIRST(A) `FIRST(A) = FIRST(A) U FIRST(B)`.
//     /// Return if FIRST(A) is changed
//     pub fn union(&mut self, a: TypeId, b: TypeId) -> bool {
//         if a == b {
//             return false;
//         }
//         let b_set = match self.0.map.remove(&b) {
//             Some(b) => b,
//             None => return false,
//         };
//         let changed = self.get_mut(a).union(b_set.as_first());
//         self.0.map.insert(b, b_set);
//         changed
//     }
// }
//
// #[derive(Debug, Default)]
// pub struct Follow<T: TokenType>(Table<T>, FollowSet<T>);
//
// impl<T: TokenType> Follow<T> {
//     // /// Get FOLLOW(T) for insertion
//     // #[inline]
//     // pub fn get_mut(&mut self, t: TypeId) -> &mut FollowSet<T> {
//     //     self.0.get_mut(t)
//     // }
//
//     // /// Get FOLLOW(T)
//     // pub fn get(&self, t: TypeId) -> &FollowSet<T> {
//     //     match self.0.map.get(&t) {
//     //         Some(set) => set.as_first(),
//     //         None => &self.1
//     //     }
//     // }
//     //
//     // #[inline]
//     // pub fn get(&self, st: TypeId)-> Cow<'_, FollowSet<T>> {
//     //     match self.map.get(&st) {
//     //         Some(set) => Cow::Borrowed(set),
//     //         None => Cow::Owned(FollowSet::default())
//     //     }
//     // }
// }


// #[derive(Debug)]
// struct Table<T: TokenType> {
//     // map: BTreeMap<TypeId, FollowSet<T>>,
// }

// impl<T: TokenType> Default for Table<T> {
//     fn default() -> Self {
//         Self {
//             map: BTreeMap::new(),
//         }
//     }
// }

// impl<T: TokenType> Table<T> {
    // /// Return if the token can follow the given SyntaxTree type
    // #[inline]
    // pub fn can_follow<'s>(&self, st: TypeId, token: Option<TokenSrc<'s, T>>) -> bool {
    //     self.map.get(&st).map(|set| set.contains(token)).unwrap_or(false)
    // }

    // #[inline]
    // pub fn get_mut(&mut self, st: TypeId)-> &mut FollowSet<T> {
    //     self.map.entry(st).or_default()
    // }

    //
    // /// Union B into A (A = A U B). Return if A is changed
    // pub fn union(&mut self, a: TypeId, b: TypeId) -> bool {
    //     if a == b {
    //         return false;
    //     }
    //     let b_set = match self.map.remove(&b) {
    //         Some(b) => b,
    //         None => return false,
    //     };
    //     let changed = self.get_mut(a).union(&b_set);
    //     self.map.insert(b, b_set);
    //     changed
    // }
    //
    // /// Union (B - e) into A (A = A U (B - e)). Return if A is changed
    // pub fn union_skip_empty(&mut self, a: TypeId, b: TypeId) -> bool {
    //     if a == b {
    //         return false;
    //     }
    //     let b_set = match self.map.remove(&b) {
    //         Some(b) => b,
    //         None => return false,
    //     };
    //     let changed = self.get_mut(a).union_skip_empty(&b_set);
    //     self.map.insert(b, b_set);
    //     changed
    // }

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
// }

