use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

use deref_derive::{Deref, DerefMut};
use derivative::Derivative;

use super::{Lexicon, Set, Token, Span, Pos};



/// Stores token and semantic information during parsing
#[derive(Derivative, Debug, Clone, PartialEq)]
#[derivative(Default(bound = "", new = "true"))]
pub struct TokenVec<L: Lexicon> {
    tokens: Vec<Cell<L>>,
}

impl<L: Lexicon> Borrow<TokenSlice<L>> for TokenVec<L> {
    fn borrow(&self) -> &TokenSlice<L> {
        TokenSlice::from_slice(&self.tokens)
    }
}

impl<L: Lexicon> BorrowMut<TokenSlice<L>> for TokenVec<L> {
    fn borrow_mut(&mut self) -> &mut TokenSlice<L> {
        TokenSlice::from_slice_mut(&mut self.tokens)
    }
}

impl<L: Lexicon> Deref for TokenVec<L> {
    type Target = TokenSlice<L>;
    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl<L: Lexicon> DerefMut for TokenVec<L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.borrow_mut()
    }
}

impl<L: Lexicon> TokenVec<L> {
    /// Add the token to the end of the storage.
    pub fn push_unchecked(&mut self, token: Token<L>) {
        self.tokens.push(token.into());
    }
}

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct TokenSlice<L: Lexicon>([Cell<L>]);

impl<L: Lexicon> Borrow<[Cell<L>]> for TokenSlice<L> {
    fn borrow(&self) -> &[Cell<L>] {
        &self.0
    }
}

impl<L: Lexicon> BorrowMut<[Cell<L>]> for TokenSlice<L> {
    fn borrow_mut(&mut self) -> &mut [Cell<L>] {
        &mut self.0
    }
}

impl<'a, L: Lexicon> IntoIterator for &'a TokenSlice<L> {
    type Item = &'a Cell<L>;
    type IntoIter = std::slice::Iter<'a, Cell<L>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, L: Lexicon> IntoIterator for &'a mut TokenSlice<L> {
    type Item = &'a mut Cell<L>;
    type IntoIter = std::slice::IterMut<'a, Cell<L>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<L: Lexicon> TokenSlice<L> {
    pub fn from_slice(slice: &[Cell<L>]) -> &Self {
        unsafe { std::mem::transmute(slice) }
    }
    pub fn from_slice_mut(slice: &mut [Cell<L>]) -> &mut Self {
        unsafe { std::mem::transmute(slice) }
    }
    /// Get the i-th token in the underlying vec
    pub fn at(&self, pos: usize) -> Option<&Cell<L>> {
        self.0.get(pos)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the tokens that overlap the span (including partially inside the span)
    ///
    /// Note that [`inside`](Self::inside) is more efficient
    /// if you know that the tokens are fully inside the span
    pub fn overlap<S: Into<Span>>(&self, span: S) -> &Self {
        match self.overlap_bounds(span) {
            Some((lo, hi)) => Self::from_slice(&self.0[lo..hi]),
            None => Self::from_slice(&[]),
        }
    }

    /// Get the tokens that overlap the span (including partially inside the span)
    ///
    /// Note that [`inside_mut`](Self::inside) is more efficient
    /// if you know that the tokens are fully inside the span
    pub fn overlap_mut<S: Into<Span>>(&mut self, span: S) -> &mut Self {
        match self.overlap_bounds(span) {
            Some((lo, hi)) => Self::from_slice_mut(&mut self.0[lo..hi]),
            None => Self::from_slice_mut(&mut []),
        }
    }

    /// Get the tokens that are fully inside the span
    pub fn inside<S: Into<Span>>(&self, span: S) -> &Self {
        match self.inside_bounds(span) {
            Some((lo, hi)) => Self::from_slice(&self.0[lo..hi]),
            None => Self::from_slice(&[]),
        }
    }

    /// Get the tokens that are fully inside the span
    pub fn inside_mut<S: Into<Span>>(&mut self, span: S) -> &mut Self {
        match self.inside_bounds(span) {
            Some((lo, hi)) => Self::from_slice_mut(&mut self.0[lo..hi]),
            None => Self::from_slice_mut(&mut []),
        }
    }

    /// Get the index range of tokens that overlap the given span. Overlapping is defined as the token's span having a non-empty intersection with the given span.
    ///
    /// The returned (lo, hi) range is such that `tokens[lo..hi]` are the tokens that overlap the given span.
    ///
    /// If there are no tokens that meet the condition, returns `None`. `tokens[lo..hi]` will not panic if
    /// `Some` is returned
    pub fn overlap_bounds<S: Into<Span>>(&self, span: S) -> Option<(usize, usize)> {
        let span = span.into();
        let lo = match self.search_lo(span.lo) {
            Ok(i) => i, // i must be overlapping/included
            Err(i) => {
                // if prev.hi == span.lo, it's not overlapping
                if i > 0 && self.0[i - 1].span.hi > span.lo {
                    i - 1
                } else {
                    i
                }
            }
        };
        let hi = match self.search_hi(span.hi) {
            Ok(i) => i + 1, // i must be overlapping/included
            Err(i) => {
                // if curr.lo == span.hi, it's not overlapping
                if i < self.0.len() && self.0[i].span.lo < span.hi {
                    i + 1
                } else {
                    i
                }
            }
        };
        if lo < hi {
            Some((lo, hi))
        } else {
            None
        }
    }

    /// Get the index range of tokens that are fully inside the given span.
    /// Inside is defined as the intersection of the token's span with the given span is the same
    /// as the token's span.
    ///
    /// The returned (lo, hi) range is such that `tokens[lo..hi]` are the tokens that are inside the given span.
    ///
    /// If there are no tokens that meet the condition, returns `None`. `tokens[lo..hi]` will not panic if
    /// `Some` is returned
    pub fn inside_bounds<S: Into<Span>>(&self, span: S) -> Option<(usize, usize)> {
        let span = span.into();
        let lo = match self.search_lo(span.lo) {
            Ok(i) => i, // i must be overlapping/included
            Err(i) => i, // i-1 is definitely not fully inside
        };
        let hi = match self.search_hi(span.hi) {
            Ok(i) => i + 1, // i must be overlapping/included
            Err(i) => i, // i is definitely not fully inside
        };
        if lo < hi {
            Some((lo, hi))
        } else {
            None
        }
    }

    /// Apply the semantic types to the tokens in this slice
    pub fn apply_semantic(&mut self, semantic: Set<L>) {
        for cell in self.0.iter_mut() {
            cell.semantic = cell.semantic.union(semantic);
        }
    }

    /// Find the largest slice of tokens such that:
    /// - The last token has `token.span.hi == pos`
    /// - All tokens in the slice satisfy `predicate`
    ///
    /// Returns `None` if there are no tokens that ends at `pos`,
    /// or the token that ends at `pos` does not satisfy the predicate
    pub fn ends_at_matches<F: Fn(&Cell<L>) -> bool>(&self, pos: Pos, predicate: F) -> Option<&Self> {
        match self.ends_at_matches_bounds(pos, predicate) {
            Some((lo, hi)) => Some(Self::from_slice(&self.0[lo..hi])),
            None => None,
        }
    }

    /// Find the largest slice of tokens such that:
    /// - The last token has `token.span.hi == pos`
    /// - All tokens in the slice satisfy `predicate`
    ///
    /// Returns `None` if there are no tokens that ends at `pos`,
    /// or the token that ends at `pos` does not satisfy the predicate
    pub fn ends_at_matches_mut<F: Fn(&Cell<L>) -> bool>(&mut self, pos: Pos, predicate: F) -> Option<&mut Self> {
        match self.ends_at_matches_bounds(pos, predicate) {
            Some((lo, hi)) => Some(Self::from_slice_mut(&mut self.0[lo..hi])),
            None => None,
        }
    }

    pub fn ends_at_matches_bounds<F: Fn(&Cell<L>) -> bool>(&self, pos: Pos, predicate: F) -> Option<(usize, usize)> {
        let hi = match self.search_hi(pos) {
            Ok(i) => i + 1,
            Err(_) => return None,
        };
        let mut lo = hi;
        while lo > 0 && predicate(&self.0[lo-1]) {
            lo -= 1;
        }
        if lo < hi {
            Some((lo, hi))
        } else {
            None
        }
    }

    /// Find the largest slice of tokens such that:
    /// - The first token has `token.span.lo == lo`
    /// - All tokens in the slice satisfy `predicate`
    ///
    /// Returns `None` if there are no tokens that ends at `pos`,
    /// or the token that ends at `pos` does not satisfy the predicate
    pub fn begins_at_matches<F: Fn(&Cell<L>) -> bool>(&self, pos: Pos, predicate: F) -> Option<&Self> {
        match self.begins_at_matches_bounds(pos, predicate) {
            Some((lo, hi)) => Some(Self::from_slice(&self.0[lo..hi])),
            None => None,
        }
    }

    /// Find the largest slice of tokens such that:
    /// - The first token has `token.span.lo == lo`
    /// - All tokens in the slice satisfy `predicate`
    ///
    /// Returns `None` if there are no tokens that ends at `pos`,
    /// or the token that ends at `pos` does not satisfy the predicate
    pub fn begins_at_matches_mut<F: Fn(&Cell<L>) -> bool>(&mut self, pos: Pos, predicate: F) -> Option<&mut Self> {
        match self.begins_at_matches_bounds(pos, predicate) {
            Some((lo, hi)) => Some(Self::from_slice_mut(&mut self.0[lo..hi])),
            None => None,
        }
    }

    pub fn begins_at_matches_bounds<F: Fn(&Cell<L>) -> bool>(&self, pos: Pos, predicate: F) -> Option<(usize, usize)> {
        let lo = match self.search_lo(pos) {
            Ok(i) => i,
            Err(_) => return None,
        };
        let mut hi = lo;
        let len = self.0.len();
        while hi < len && predicate(&self.0[hi]) {
            hi += 1;
        }
        if lo < hi {
            Some((lo, hi))
        } else {
            None
        }
    }

    /// Return the greatest `i` where `tokens[i].lo >= lo`, or 0.
    /// Result is ok if the equal condition is true
    pub fn search_lo(&self, lo: Pos) -> Result<usize, usize> {
        self.0.binary_search_by(|cell| cell.token.span.lo.cmp(&lo))
    }

    /// Return the least `i` where `tokens[i].hi >= hi`, or `tokens.len()`
    /// Result is ok if the equal condition is true
    pub fn search_hi(&self, hi: Pos) -> Result<usize, usize> {
        self.0.binary_search_by(|cell| cell.token.span.hi.cmp(&hi))
    }

    pub fn as_slice(&self) -> &[Cell<L>] {
        &self.0
    }

    pub fn as_slice_mut(&mut self) -> &mut [Cell<L>] {
        &mut self.0
    }
}


/// One cell in a [`TokenVec`], with the token and any semantic types added
#[derive(Deref, DerefMut, Clone, PartialEq, Eq)]
pub struct Cell<L: Lexicon> {
    #[deref]
    token: Token<L>,
    semantic: Set<L>,
}

impl<L: Lexicon> std::fmt::Debug for Cell<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}+{:?}", self.token, self.semantic)
    }
}

impl<L: Lexicon> From<Token<L>> for Cell<L> {
    fn from(token: Token<L>) -> Self {
        Self {
            token,
            semantic: Set::new(),
        }
    }
}

impl<L: Lexicon> From<Cell<L>> for Token<L> {
    fn from(cell: Cell<L>) -> Self {
        cell.token
    }
}

impl<L: Lexicon> Cell<L> {
    /// Get the semantic types and the original token type
    pub fn types(&self) -> Set<L> {
        let mut s = self.semantic;
        s.insert(self.token.ty);
        s
    }

    /// Get only the semantic types, not the original token type
    ///
    /// (It's possible that the semantic types include the original token type)
    pub fn semantics(&self) -> Set<L> {
        self.semantic
    }

    /// Get the semantic set for mutation
    pub fn semantics_mut(&mut self) -> &mut Set<L> {
        &mut self.semantic
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::token_set;
    use crate::test::TestTokenType as T;

    fn example_empty() -> TokenVec<T> {
        TokenVec::new()
    }

    fn example() -> TokenVec<T> {
        let mut vec = TokenVec::new();
        vec.push_unchecked(Token::new(1..2, T::B)); // 0      E
        vec.push_unchecked(Token::new(3..4, T::C)); // 1    D E
        vec.push_unchecked(Token::new(4..5, T::C)); // 2  A D E
        vec.push_unchecked(Token::new(7..9, T::B)); // 3  A D E
        vec.push_unchecked(Token::new(11..13, T::C)); // 4  D E
        vec.push_unchecked(Token::new(15..17, T::C)); // 5    E
        vec.inside_mut(4..9).apply_semantic(token_set!(T { A }));
        vec.inside_mut(3..13).apply_semantic(token_set!(T { D }));
        vec.inside_mut(1..17).apply_semantic(token_set!(T { E }));
        vec
    }

    #[test]
    fn overlap_empty_input() {
        let vec = example();
        assert!(vec.overlap(2..2).is_empty());
        assert!(vec.overlap(7..7).is_empty());

        let vec = example_empty();
        assert!(vec.overlap(0..0).is_empty());
        assert!(vec.overlap(0..1).is_empty());
        assert!(vec.overlap(1..2).is_empty());
        assert!(vec.overlap(2..2).is_empty());
        assert!(vec.overlap(0..2).is_empty());
        assert!(vec.inside(2..2).is_empty());
        assert!(vec.inside(7..7).is_empty());
    }

    #[test]
    fn overlap_none() {
        let vec = example();
        assert!(vec.overlap(2..3).is_empty());
        assert!(vec.overlap(9..10).is_empty());
        assert!(vec.inside(2..3).is_empty());
        assert!(vec.inside(9..10).is_empty());
    }

    // matrix: fully inside, partially inside, fully outside, fully outside touching

    #[test]
    fn both_end_fully_inside() {
        let vec = example();
        assert_eq!(vec.overlap(1..9).as_slice(), &vec.as_slice()[0..4]);
        assert_eq!(vec.inside(1..9).as_slice(), &vec.as_slice()[0..4]);
    }

    #[test]
    fn left_fully_inside_right_partially_inside() {
        let vec = example();
        assert_eq!(vec.overlap(3..8).as_slice(), &vec.as_slice()[1..4]);
        assert_eq!(vec.inside(3..8).as_slice(), &vec.as_slice()[1..3]);
    }

    #[test]
    fn left_fully_inside_right_fully_outside() {
        let vec = example();
        assert_eq!(vec.overlap(1..10).as_slice(), &vec.as_slice()[0..4]);
        assert_eq!(vec.inside(1..10).as_slice(), &vec.as_slice()[0..4]);
    }

    #[test]
    fn left_fully_inside_right_touching() {
        let vec = example();
        assert_eq!(vec.overlap(1..15).as_slice(), &vec.as_slice()[0..5]);
        assert_eq!(vec.inside(1..15).as_slice(), &vec.as_slice()[0..5]);
    }

    #[test]
    fn left_fully_inside_right_over_max() {
        let vec = example();
        assert_eq!(vec.overlap(7..100).as_slice(), &vec.as_slice()[3..]);
        assert_eq!(vec.inside(7..100).as_slice(), &vec.as_slice()[3..]);
    }

    #[test]
    fn left_partially_inside_right_fully_inside() {
        let vec = example();
        assert_eq!(vec.overlap(8..13).as_slice(), &vec.as_slice()[3..5]);
        assert_eq!(vec.inside(8..13).as_slice(), &vec.as_slice()[4..5]);
    }

    #[test]
    fn both_partially_inside() {
        let vec = example();
        assert_eq!(vec.overlap(8..16).as_slice(), &vec.as_slice()[3..6]);
        assert_eq!(vec.inside(8..16).as_slice(), &vec.as_slice()[4..5]);
        assert!(vec.inside(8..12).is_empty());
    }

    #[test]
    fn left_partially_inside_right_fully_outside() {
        let vec = example();
        assert_eq!(vec.overlap(8..14).as_slice(), &vec.as_slice()[3..5]);
        assert_eq!(vec.inside(8..14).as_slice(), &vec.as_slice()[4..5]);
    }

    #[test]
    fn left_partially_inside_right_touching() {
        let vec = example();
        assert_eq!(vec.overlap(8..15).as_slice(), &vec.as_slice()[3..5]);
        assert_eq!(vec.inside(8..15).as_slice(), &vec.as_slice()[4..5]);
    }

    #[test]
    fn left_partially_inside_right_over_max() {
        let vec = example();
        assert_eq!(vec.overlap(12..100).as_slice(), &vec.as_slice()[4..]);
        assert_eq!(vec.inside(12..100).as_slice(), &vec.as_slice()[5..]);
    }

    #[test]
    fn left_fully_outside_right_fully_inside() {
        let vec = example();
        assert_eq!(vec.overlap(6..13).as_slice(), &vec.as_slice()[3..5]);
        assert_eq!(vec.inside(6..13).as_slice(), &vec.as_slice()[3..5]);
    }

    #[test]
    fn left_fully_outside_right_partially_inside() {
        let vec = example();
        assert_eq!(vec.overlap(6..12).as_slice(), &vec.as_slice()[3..5]);
        assert_eq!(vec.inside(6..12).as_slice(), &vec.as_slice()[3..4]);
    }

    #[test]
    fn both_fully_outside() {
        let vec = example();
        assert_eq!(vec.overlap(6..14).as_slice(), &vec.as_slice()[3..5]);
        assert_eq!(vec.inside(6..14).as_slice(), &vec.as_slice()[3..5]);
    }

    #[test]
    fn left_fully_outside_right_touching() {
        let vec = example();
        assert_eq!(vec.overlap(6..15).as_slice(), &vec.as_slice()[3..5]);
        assert_eq!(vec.inside(6..15).as_slice(), &vec.as_slice()[3..5]);
    }

    #[test]
    fn left_fully_outside_right_over_max() {
        let vec = example();
        assert_eq!(vec.overlap(6..100).as_slice(), &vec.as_slice()[3..]);
        assert_eq!(vec.inside(6..100).as_slice(), &vec.as_slice()[3..]);
    }

    #[test]
    fn left_fully_outside_touching_right_fully_inside() {
        let vec = example();
        assert_eq!(vec.overlap(2..13).as_slice(), &vec.as_slice()[1..5]);
        assert_eq!(vec.inside(2..13).as_slice(), &vec.as_slice()[1..5]);
    }

    #[test]
    fn left_full_outside_touching_right_partially_inside() {
        let vec = example();
        assert_eq!(vec.overlap(2..12).as_slice(), &vec.as_slice()[1..5]);
        assert_eq!(vec.inside(2..12).as_slice(), &vec.as_slice()[1..4]);
    }

    #[test]
    fn left_full_outside_touching_right_fully_outside() {
        let vec = example();
        assert_eq!(vec.overlap(2..10).as_slice(), &vec.as_slice()[1..4]);
        assert_eq!(vec.inside(2..10).as_slice(), &vec.as_slice()[1..4]);
    }

    #[test]
    fn left_full_outside_touching_right_fully_outside_touching() {
        let vec = example();
        assert_eq!(vec.overlap(2..15).as_slice(), &vec.as_slice()[1..5]);
        assert_eq!(vec.inside(2..15).as_slice(), &vec.as_slice()[1..5]);
    }

    #[test]
    fn left_full_outside_touching_right_over_max() {
        let vec = example();
        assert_eq!(vec.overlap(2..100).as_slice(), &vec.as_slice()[1..]);
        assert_eq!(vec.inside(2..100).as_slice(), &vec.as_slice()[1..]);
    }

    #[test]
    fn left_over_min_right_fully_inside() {
        let vec = example();
        assert_eq!(vec.overlap(0..13).as_slice(), &vec.as_slice()[0..5]);
        assert_eq!(vec.inside(0..13).as_slice(), &vec.as_slice()[0..5]);
    }

    #[test]
    fn left_over_min_right_partially_inside() {
        let vec = example();
        assert_eq!(vec.overlap(0..8).as_slice(), &vec.as_slice()[0..4]);
        assert_eq!(vec.inside(0..8).as_slice(), &vec.as_slice()[0..3]);
    }

    #[test]
    fn left_over_min_right_fully_outside() {
        let vec = example();
        assert_eq!(vec.overlap(0..6).as_slice(), &vec.as_slice()[0..3]);
        assert_eq!(vec.inside(0..6).as_slice(), &vec.as_slice()[0..3]);
    }

    #[test]
    fn left_over_min_right_touching() {
        let vec = example();
        assert_eq!(vec.overlap(0..11).as_slice(), &vec.as_slice()[0..4]);
        assert_eq!(vec.inside(0..11).as_slice(), &vec.as_slice()[0..4]);
    }

    #[test]
    fn left_over_min_right_over_max() {
        let vec = example();
        assert_eq!(vec.overlap(0..100).as_slice(), &vec.as_slice()[..]);
        assert_eq!(vec.inside(0..100).as_slice(), &vec.as_slice()[..]);
    }

    #[test]
    fn search_lo() {
        let vec = example();
        assert_eq!(vec.search_lo(0), Err(0));
        assert_eq!(vec.search_lo(4), Ok(2));
        assert_eq!(vec.search_lo(8), Err(4));
        assert_eq!(vec.search_lo(5), Err(3));
        assert_eq!(vec.search_lo(14), Err(5));
        assert_eq!(vec.search_lo(16), Err(6));
    }

    #[test]
    fn search_hi() {
        let vec = example();
        assert_eq!(vec.search_hi(0), Err(0));
        assert_eq!(vec.search_hi(1), Err(0));
        assert_eq!(vec.search_hi(2), Ok(0));
        assert_eq!(vec.search_hi(4), Ok(1));
        assert_eq!(vec.search_hi(6), Err(3));
        assert_eq!(vec.search_hi(13), Ok(4));
        assert_eq!(vec.search_hi(18), Err(6));
    }

    #[test]
    fn begin_none() {
        let vec = example();
        assert_eq!(vec.begins_at_matches(5, |_| {
            panic!("should not be called");
        }), None);
        assert_eq!(vec.begins_at_matches(8, |_| {
            panic!("should not be called");
        }), None);
        assert_eq!(vec.begins_at_matches(11, |x| {
            x.semantics().contains(T::G)
        }), None);
    }

    #[test]
    fn begin_single() {
        let vec = example();
        let result = vec.begins_at_matches(7, |x| x.semantics().contains(T::A)).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[3..4]);
    }

    #[test]
    fn begin_many() {
        let vec = example();
        let result = vec.begins_at_matches(7, |x| x.semantics().contains(T::D)).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[3..5]);
    }

    #[test]
    fn begin_to_end() {
        let vec = example();
        let result = vec.begins_at_matches(7, |x| x.semantics().contains(T::E)).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[3..]);
        let result = vec.begins_at_matches(7, |_| true).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[3..]);
    }

    #[test]
    fn end_none() {
        let vec = example();
        assert_eq!(vec.ends_at_matches(7, |_| {
            panic!("should not be called");
        }), None);
        assert_eq!(vec.ends_at_matches(10, |_| {
            panic!("should not be called");
        }), None);
        assert_eq!(vec.ends_at_matches(13, |x| {
            x.semantics().contains(T::G)
        }), None);
    }

    #[test]
    fn end_single() {
        let vec = example();
        let result = vec.ends_at_matches(5, |x| x.semantics().contains(T::A)).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[2..3]);
    }

    #[test]
    fn end_many() {
        let vec = example();
        let result = vec.ends_at_matches(5, |x| x.semantics().contains(T::D)).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[1..3]);
    }

    #[test]
    fn end_from_begin() {
        let vec = example();
        let result = vec.ends_at_matches(5, |x| x.semantics().contains(T::E)).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[..3]);
        let result = vec.ends_at_matches(5, |_| true).unwrap().as_slice();
        assert_eq!(result, &vec.as_slice()[..3]);
    }
    
}
