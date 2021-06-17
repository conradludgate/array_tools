//! Modify simple arrays
//!
//! ```
//! use array_tools::ArrayIterator;
//! let a = [1, 2, 3, 4];
//! let b = [5, 6, 7, 8];
//! let c = a.zip_array(b).map_array(|(a, b)| a + b).collect_array();
//! assert_eq!(c, [6, 8, 10, 12]);
//! ```
#![no_std]
#![deny(missing_docs)]

/// Similar to [`Iterator`], `ArrayIterator` is an iterator over constant
/// sized arrays
pub trait ArrayIterator<const N: usize>: Sized {
    /// Item returned by this `ArrayIterator`
    type Item;

    /// Lazily mutate the contents of an array
    fn map_array<U, F: Fn(Self::Item) -> U>(self, f: F) -> Map<Self, F> {
        Map { a: self, f }
    }
    /// Lazily combine two `ArrayIterator`s into an new one
    fn zip_array<B: ArrayIterator<N>>(self, b: B) -> Zip<Self, B> {
        Zip { a: self, b }
    }

    /// Collect the contents of this `ArrayIterator` into an array
    ///
    /// This is very similar to [`build_array`](ArrayIterator::build_array) but ignores
    /// the case that any drop handlers need to be called
    fn collect_array(self) -> [Self::Item; N]
    where
        Self::Item: Copy,
    {
        // Safety:
        // Self::Item is Copy, so it has no Drop impl,
        // and we will not read from array until all values are set
        // So this is safe
        let mut array: [Self::Item; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            array[i] = self.get(i);
        }
        array
    }

    /// Build the contents of this `ArrayIterator` into an array
    ///
    /// This is very similar to [`collect_array`](ArrayIterator::collect_array) but explicitly handles the
    /// case where the function panics and items need to be dropped
    #[cfg(feature = "drop")]
    fn build_array(self) -> [Self::Item; N]
    where
        Self::Item: Drop,
    {
        let mut builder = array_builder::ArrayBuilder::new();
        for i in 0..N {
            builder.push(self.get(i));
        }
        // Safety: we have filled all N spots, so this is safe
        unsafe { builder.build_unchecked() }
    }

    /// Separate and collect the contents of the `ArrayIterator` into two arrays
    fn unzip_array<A, B>(self) -> ([A; N], [B; N])
    where
        Self: ArrayIterator<N, Item = (A, B)>,
        A: Copy,
        B: Copy,
    {
        // Safety:
        // Self::Item is Copy, so it has no Drop impl,
        // and we will not read from array until all values are set
        // So this is safe
        let mut array1: [A; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut array2: [B; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        for i in 0..N {
            let (a, b) = self.get(i);
            array1[i] = a;
            array2[i] = b;
        }
        (array1, array2)
    }

    /// Get a single item out of this `ArrayIterator`
    ///
    /// Panics:
    /// This is expected to panic if `n` >= `N`
    fn get(&self, n: usize) -> Self::Item;
}

/// Implementation behind [`ArrayIterator::map_array`]
pub struct Map<A, F> {
    a: A,
    f: F,
}

/// Implementation behind [`ArrayIterator::zip_array`]
pub struct Zip<A, B> {
    a: A,
    b: B,
}

impl<T: Copy, const N: usize> ArrayIterator<N> for [T; N] {
    type Item = T;
    fn collect_array(self) -> [T; N] {
        self
    }
    fn get(&self, n: usize) -> T {
        self[n]
    }
}

impl<U, A, F, const N: usize> ArrayIterator<N> for Map<A, F>
where
    U: Copy,
    A: ArrayIterator<N>,
    F: Fn(A::Item) -> U,
{
    type Item = U;
    fn get(&self, n: usize) -> U {
        (self.f)(self.a.get(n))
    }
}

impl<A, B, const N: usize> ArrayIterator<N> for Zip<A, B>
where
    A: ArrayIterator<N>,
    B: ArrayIterator<N>,
{
    type Item = (A::Item, B::Item);
    fn get(&self, n: usize) -> Self::Item {
        (self.a.get(n), self.b.get(n))
    }
}

#[cfg(test)]
mod tests {
    use crate::ArrayIterator;

    #[test]
    fn zip() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let c = a.zip_array(b).map_array(|(a, b)| a + b).collect_array();
        assert_eq!(c, [6, 8, 10, 12]);
    }

    #[test]
    fn unzip() {
        let a = [0, 1, 2, 3];
        let (div, rem) = a.map_array(|a| (a / 2, a % 2)).unzip_array();
        assert_eq!(div, [0, 0, 1, 1]);
        assert_eq!(rem, [0, 1, 0, 1]);
    }
}
