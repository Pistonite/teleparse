use std::ops::Range;
use std::fmt::Debug;

/// Position in the source code
pub type Pos = usize;

///////////////////////////////////////////////////////////
// Span
///////////////////////////////////////////////////////////

/// A span of source code
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// Start of the span, inclusive
    pub lo: Pos,
    /// End of the span, exclusive
    pub hi: Pos,
}

impl From<Range<Pos>> for Span {
    #[inline]
    fn from(range: Range<Pos>) -> Self {
        Self {
            lo: range.start,
            hi: range.end,
        }
    }
}

impl From<(Pos, Pos)> for Span {
    #[inline]
    fn from((lo, hi): (Pos, Pos)) -> Self {
        Self { lo, hi }
    }
}

impl Span {
    /// Create a new span
    #[inline]
    pub fn new(lo: Pos, hi: Pos) -> Self {
        Self { lo, hi }
    }

    /// Get the content of this span from the entire source input
    pub fn get<'s>(&self, input: &'s str) -> &'s str {
        if self.hi <= self.lo {
            return "";
        }
        let hi = self.hi.min(input.len());
        &input[self.lo..hi]
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hi <= self.lo {
            write!(f, "{}", self.lo)
        } else {
            write!(f, "{}..{}", self.lo, self.hi)
        }
    }
}

///////////////////////////////////////////////////////////
// ToSpan
///////////////////////////////////////////////////////////

/// Trait for types that can be converted to a [`Span`]
///
/// [`Token`]s and derived syntax nodes all implement this trait
pub trait ToSpan {
    fn lo(&self) -> Pos;
    fn hi(&self) -> Pos;
    fn span(&self) -> Span {
        Span::new(self.lo(), self.hi())
    }
}
pub use teleparse_macros::ToSpan;

impl ToSpan for Span {
    fn lo(&self) -> Pos {
        self.lo
    }
    fn hi(&self) -> Pos {
        self.hi
    }
    fn span(&self) -> Span {
        *self
    }
}

macro_rules! derive_to_span_tuple {
    ($last:tt, $($n:ident),*) => {
        impl<$($n: ToSpan),*> ToSpan for ($($n,)*) {
            #[inline]
            fn lo(&self) -> Pos {
                self.0.lo()
            }
            #[inline]
            fn hi(&self) -> Pos {
                self.$last.hi()
            }
        }
    };
}

derive_to_span_tuple!(1, A, B);
derive_to_span_tuple!(2, A, B, C);
derive_to_span_tuple!(3, A, B, C, D);
derive_to_span_tuple!(4, A, B, C, D, E);
