use std::borrow::{Borrow, BorrowMut, Cow};
use std::collections::btree_map::Entry;
use std::fmt::{self, Debug, Formatter};
use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};

use crate::TokenType;


// pub struct Table<T: TokenType> {
//     cache: BTreeSet<TypeId>,
//     map: BTreeMap<TypeId, TermSet<T>>,
// }

/// A pool of literal constants to efficiently store literal constants
#[derive(Default, Debug, Clone)]
pub struct LitPool {
    map: BTreeMap<String, &'static str>,
}

impl LitPool {
    pub fn get(&mut self, lit: &'static str) -> &'static str {
        if self.map.get(lit).is_none() {
            self.map.insert(lit.to_string(), lit);
        }
        lit
    }
}

