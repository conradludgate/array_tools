//! ArrayIterator implmentations on [`Zip`]

use crate::ArrayIterator;

/// Implementation behind [`ArrayIterator::zip`]
pub struct Zip<A, B> {
    a: A,
    b: B,
}

impl<A, B> Zip<A, B> {
    /// Create a new Zip. See [`ArrayIterator::zip`]
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B, const N: usize> ArrayIterator<N> for Zip<A, B>
where
    A: ArrayIterator<N>,
    B: ArrayIterator<N>,
{
    type Item = (A::Item, B::Item);
    unsafe fn next(&mut self) -> Self::Item {
        (self.a.next(), self.b.next())
    }
}
