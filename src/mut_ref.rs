//! ArrayIterator implementations on &mut [T; N]

use crate::{ArrayIterator, IntoArrayIterator};
use core::marker::PhantomData;

/// ArrayIter is an implementation of [`ArrayIterator`] for &[T; N]
pub struct ArrayIter<'a, T: 'a, const N: usize> {
    ptr: *mut T,
    i: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T, const N: usize> IntoArrayIterator<N> for &'a mut [T; N] {
    type Item = &'a mut T;
    type ArrayIter = ArrayIter<'a, T, N>;
    fn into_array_iter(self) -> Self::ArrayIter {
        ArrayIter {
            ptr: self.as_mut_ptr(),
            i: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, const N: usize> ArrayIterator<N> for ArrayIter<'a, T, N> {
    type Item = &'a mut T;
    unsafe fn next(&mut self) -> Self::Item {
        debug_assert_ne!(self.i, N, "Called next too many times");
        let n = self.i + 1;
        &mut *self.ptr.add(core::mem::replace(&mut self.i, n))
    }
}
