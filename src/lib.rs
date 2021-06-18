//! Modify simple arrays
//!
//! ```
//! use array_iter_tools::{ArrayIterator, IntoArrayIterator};
//! let a = [1, 2, 3, 4];
//! let b = [5, 6, 7, 8];
//! let c = a.into_array_iter().zip(b).map(|(a, b)| a + b).collect();
//! assert_eq!(c, [6, 8, 10, 12]);
//! ```
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod array;
pub mod reference;
pub mod mut_ref;

pub mod map;
pub mod zip;

use map::Map;
use zip::Zip;

/// Similar to [`IntoIterator`], `IntoArrayIterator` allows types to become [`ArrayIterator`]s
pub trait IntoArrayIterator<const N: usize> {
    /// Item returned by the `ArrayIterator`
    type Item;
    /// The [`ArrayIterator`] returned
    type ArrayIter: ArrayIterator<N, Item = Self::Item>;

    /// Converts self into an [`ArrayIterator`]
    fn into_array_iter(self) -> Self::ArrayIter;
}

/// Similar to [`Iterator`], `ArrayIterator` is an iterator over constant sized arrays
///
/// ```
/// use array_iter_tools::{ArrayIterator, IntoArrayIterator};
/// let a = [1, 2, 3, 4];
/// let b = [5, 6, 7, 8];
/// let c = a.into_array_iter().zip(b).map(|(a, b)| a + b).collect();
/// assert_eq!(c, [6, 8, 10, 12]);
/// ```
pub trait ArrayIterator<const N: usize>: Sized {
    /// Item returned by this `ArrayIterator`
    type Item;

    /// Lazily mutate the contents of an array
    fn map<U, F: Fn(Self::Item) -> U>(self, f: F) -> Map<Self, F> {
        Map::new(self, f)
    }
    /// Lazily combine two `ArrayIterator`s into an new one
    fn zip<B: IntoArrayIterator<N>>(self, b: B) -> Zip<Self, B::ArrayIter> {
        Zip::new(self, b.into_array_iter())
    }

    /// Collect the contents of this `ArrayIterator` into an array
    ///
    /// This is very similar to [`build_array`](ArrayIterator::build_array) but ignores
    /// the case that any drop handlers need to be called
    fn collect(mut self) -> [Self::Item; N] {
        let mut builder = array_builder::ArrayBuilder::new();
        for _ in 0..N {
            builder.push(unsafe { self.next() });
        }
        unsafe { builder.build_unchecked() }
    }

    /// Separate and collect the contents of the `ArrayIterator` into two arrays
    fn unzip<A, B>(mut self) -> ([A; N], [B; N])
    where
        Self: ArrayIterator<N, Item = (A, B)>,
    {
        let mut builder1 = array_builder::ArrayBuilder::new();
        let mut builder2 = array_builder::ArrayBuilder::new();
        for _ in 0..N {
            let (a, b) = unsafe { self.next() };
            builder1.push(a);
            builder2.push(b);
        }
        unsafe { (builder1.build_unchecked(), builder2.build_unchecked()) }
    }

    /// Get the next item out of this `ArrayIterator`
    ///
    /// Safety:
    /// This function is used internally by [`collect`](ArrayIterator::collect) and [`unzip`](ArrayIterator::unzip)
    /// Calling next and then calling one of those two functions is undefined behaviour.
    ///
    /// Calling next > N times is also undefined behaviour.
    unsafe fn next(&mut self) -> Self::Item;
}

impl<A: ArrayIterator<N> + Sized, const N: usize> IntoArrayIterator<N> for A {
    type Item = A::Item;
    type ArrayIter = A;
    fn into_array_iter(self) -> Self::ArrayIter {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArrayIterator, IntoArrayIterator};

    #[test]
    fn zip() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let c = a.into_array_iter().zip(b).map(|(a, b)| a + b).collect();
        assert_eq!(c, [6, 8, 10, 12]);
    }

    #[test]
    fn unzip() {
        let a = [0, 1, 2, 3];
        let (div, rem) = a.into_array_iter().map(|a| (a / 2, a % 2)).unzip();
        assert_eq!(div, [0, 0, 1, 1]);
        assert_eq!(rem, [0, 1, 0, 1]);
    }

    #[test]
    fn mut_array() {
        let a = [0, 1, 2, 3];
        let mut b = [5, 6, 7, 8];
        let a = a.into_array_iter().zip(&mut b).map(|(a, b)| {
            core::mem::replace(b, a)
        }).collect();
        assert_eq!(a, [5, 6, 7, 8]);
        assert_eq!(b, [0, 1, 2, 3]);
    }
}
