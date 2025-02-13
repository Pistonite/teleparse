use std::borrow::{Borrow, BorrowMut};

use derivative::Derivative;

use super::Lexicon;

/// Map of token type to items
///
/// This uses the internal storage derived with [`Lexicon`], but
/// wraps it in a map-like API for easy access.
#[derive(Derivative, Clone)]
#[derivative(Default(new = "true", bound = ""))]
pub struct Map<L: Lexicon, T: Default + Clone>(L::Map<T>);

impl<L: Lexicon, T: Default + Clone + std::fmt::Debug> std::fmt::Debug for Map<L, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_map();
        let slice: &[T] = self.0.borrow();
        for (ty, item) in slice.iter().enumerate() {
            s.entry(&L::from_id_unchecked(ty), item);
        }
        s.finish()
    }
}

impl<L: Lexicon, T: Default + Clone + PartialEq> PartialEq for Map<L, T> {
    fn eq(&self, other: &Self) -> bool {
        std::iter::zip(self.0.borrow().iter(), other.0.borrow().iter()).all(|(a, b)| a == b)
    }
}

impl<L: Lexicon, T: Default + Clone> Map<L, T> {
    /// Get the item for a token type
    #[inline]
    pub fn get(&self, ty: L) -> &T {
        &self.0.borrow()[ty.id()]
    }

    /// Get the mutable item for a token type
    #[inline]
    pub fn get_mut(&mut self, ty: L) -> &mut T {
        &mut self.0.borrow_mut()[ty.id()]
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.borrow().iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.borrow_mut().iter_mut()
    }

    #[inline]
    pub fn iter_zip(&self) -> impl Iterator<Item = (L, &T)> {
        self.iter()
            .enumerate()
            .map(|(ty, item)| (L::from_id_unchecked(ty), item))
    }

    #[inline]
    pub fn iter_zip_mut(&mut self) -> impl Iterator<Item = (L, &mut T)> {
        self.iter_mut()
            .enumerate()
            .map(|(ty, item)| (L::from_id_unchecked(ty), item))
    }
}
