use std::borrow::{Borrow, BorrowMut};

use derivative::Derivative;

use crate::TokenType;

/// Map of token type to items
///
/// The derived implementation of [`TokenType`] uses array for the map implementation
/// so it's efficient.
#[derive(Derivative, Clone)]
#[derivative(Default(new="true", bound=""))]
pub struct Map<T: TokenType, TItem: Default + Clone>(T::Map<TItem>);

impl<T: TokenType, TItem: Default + Clone + std::fmt::Debug> std::fmt::Debug for Map<T, TItem> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Map");
        let slice: &[TItem] = self.0.borrow();
        for (ty,item) in slice.iter().enumerate() {
            s.field(&ty.to_string(), item);
        }
        s.finish()
    }
}

impl<T: TokenType, TItem: Default + Clone> Map<T, TItem> {
    /// Get the item for a token type
    #[inline]
    pub fn get(&self, ty: T) -> &TItem {
        &self.0.borrow()[ty.id()]
    }

    /// Get the mutable item for a token type
    #[inline]
    pub fn get_mut(&mut self, ty: T) -> &mut TItem {
        &mut self.0.borrow_mut()[ty.id()]
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &TItem> {
        self.0.borrow().iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TItem> {
        self.0.borrow_mut().iter_mut()
    }

    #[inline]
    pub fn iter_zip(&self) -> impl Iterator<Item = (T, &TItem)> {
        self.iter().enumerate().map(|(ty, item)| (T::from_id(ty), item))
    }

    #[inline]
    pub fn iter_zip_mut(&mut self) -> impl Iterator<Item = (T, &mut TItem)> {
        self.iter_mut().enumerate().map(|(ty, item)| (T::from_id(ty), item))
    }
}
