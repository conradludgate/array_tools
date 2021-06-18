//! ArrayIterator implementations on &[T; N]

use crate::{ArrayIterator, IntoArrayIterator};

/// ArrayIter is a drop safe implementation of [`ArrayIterator`] for [T; N]
pub struct ArrayIter<'a, T, const N: usize> {
    data: &'a [T; N],
    i: usize,
}

impl<'a, T, const N: usize> IntoArrayIterator<N> for &'a [T; N] {
    type Item = &'a T;
    type ArrayIter = ArrayIter<'a, T, N>;
    fn into_array_iter(self) -> Self::ArrayIter {
        ArrayIter {
            data: self,
            i: 0,
        }
    }
}

impl<'a, T, const N: usize> ArrayIterator<N> for ArrayIter<'a, T, N> {
    type Item = &'a T;
    unsafe fn next(&mut self) -> Self::Item {
        debug_assert_ne!(self.i, N, "Called next too many times");
        let n = self.i + 1;
        self.data.get_unchecked(core::mem::replace(&mut self.i, n))
    }
}
