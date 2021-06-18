//! ArrayIterator implmentations on [`Map`]

use crate::ArrayIterator;

/// Implementation behind [`ArrayIterator::map`]
pub struct Map<A, F> {
    a: A,
    f: F,
}

impl<A, F> Map<A, F> {
    /// Create a new Map. See [`ArrayIterator::map`]
    pub fn new(a: A, f: F) -> Self {
        Self { a, f }
    }
}

impl<U, A, F, const N: usize> ArrayIterator<N> for Map<A, F>
where
    A: ArrayIterator<N>,
    F: Fn(A::Item) -> U,
{
    type Item = U;
    unsafe fn next(&mut self) -> U {
        (self.f)(self.a.next())
    }
}
