
use std::borrow::{Borrow, BorrowMut};
use std::fmt::{self, Debug, Formatter};

use crate::token::TokenSrc;
use crate::TokenType;

use super::LitSet;

// impl<T: TokenType> FirstSet<T> {
//
//
//     /// Union other into self (self = self U other). Return if self is changed
//     #[inline]
//     pub fn union(&mut self, other: &Self) -> bool {
//         let changed = if !self.contains_empty {
//             self.contains_empty = other.contains_empty;
//             other.contains_empty
//         } else {
//             false
//         };
//         union_impl::<T>(&mut self.array, &other.array) || changed
//     }
//
//     pub fn intersects(&self, other: &Self) -> bool {
//         todo!()
//         // if self.contains_empty && other.contains_empty {
//         //     return true;
//         // }
//         // for (set, other_set) in self.array.borrow().iter().zip(other.array.borrow().iter()) {
//         //     if set.intersects(other_set) {
//         //         return true;
//         //     }
//         // }
//         // return false;
//     }
//
// }
//
// #[derive(Clone)]
// pub struct FollowSet<T: TokenType>(FirstSet<T>);
//
// impl<T: TokenType> Default for FollowSet<T> {
//     fn default() -> Self {
//         Self(FirstSet::default())
//     }
// }
//
// impl<T: TokenType> Debug for FollowSet<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         self.0.fmt(f)
//     }
// }
//
//
// impl<T: TokenType> FollowSet<T> {
//     pub fn contains<'s>(&self, token: Option<TokenSrc<'s, T>>) -> bool {
//         self.0.contains(token)
//     }
//
//     #[inline]
//     pub fn insert_eof(&mut self) -> bool {
//         self.0.insert_epsilon()
//     }
//
//     /// Union with another FOLLOW set
//     #[inline]
//     pub fn union(&mut self, other: &Self) -> bool {
//         self.0.union(&other.0)
//     }
//
//     /// Union with a FIRST set. (Epsilon will not be included)
//     #[inline]
//     pub fn union_first(&mut self, first: &FirstSet<T>) -> bool {
//         union_impl::<T>(&mut self.0.array, &first.array)
//     }
//
//     #[inline]
//     pub fn as_first(&self) -> &FirstSet<T> {
//         &self.0
//     }
//
//     #[inline]
//     pub fn as_first_mut(&mut self) -> &mut FirstSet<T> {
//         &mut self.0
//     }
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // #[test]
//     // fn todo() {
//     //     todo!()
//     // }
// }
