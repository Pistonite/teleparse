use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet};

use crate::SyntaxTree;

macro_rules! delegate_impl {
    (pub fn $name:ident(&self $(,$arg:ident : $arg_t:ty)*) -> $ret:ty) => {
        #[inline(always)]
        pub fn $name<ST: $crate::SyntaxTree>(&self $(,$arg:$arg_t)*) -> $ret {
            let t = ST::type_id();
            self.0.$name(&t $(,$arg)*)
        }
    };
    (pub fn $name:ident(&mut self $(,$arg:ident : $arg_t:ty)*) -> $ret:ty) => {
        #[inline(always)]
        pub fn $name<ST: $crate::SyntaxTree>(&mut self $(,$arg:$arg_t)*) -> $ret {
            let t = ST::type_id();
            self.0.$name(&t $(,$arg)*)
        }
    };
}


pub struct ProdMap<T>(BTreeMap<TypeId, T>);
impl<T> ProdMap<T> {
    delegate_impl!(pub fn get(&self) -> Option<&T>);
    delegate_impl!(pub fn get_mut(&mut self) -> Option<&mut T>);
}
pub struct ProdSet(BTreeSet<TypeId>);
