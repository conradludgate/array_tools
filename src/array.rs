//! ArrayIterator implementations on [T; N]

use crate::{ArrayIterator, IntoArrayIterator};
use core::mem::MaybeUninit;

/// ArrayIter is a drop safe implementation of [`ArrayIterator`] for [T; N]
pub struct ArrayIter<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    i: usize,
}

impl<T, const N: usize> IntoArrayIterator<N> for [T; N] {
    type Item = T;
    type ArrayIter = ArrayIter<T, N>;
    fn into_array_iter(self) -> Self::ArrayIter {
        let iter = ArrayIter {
            data: unsafe { core::mem::transmute_copy(&self) },
            i: 0,
        };
        core::mem::forget(self);
        iter
    }
}

impl<T, const N: usize> ArrayIterator<N> for ArrayIter<T, N> {
    type Item = T;
    fn collect(self) -> [T; N] {
        debug_assert_eq!(
            self.i, 0,
            "Called collect after calling next, breaking the safety contract specified by next"
        );
        let array = unsafe { core::mem::transmute_copy(&self.data) };
        core::mem::forget(self);
        array
    }
    unsafe fn next(&mut self) -> Self::Item {
        debug_assert_ne!(self.i, N, "Called next too many times");
        let n = self.i + 1;
        self.data
            .get_unchecked(core::mem::replace(&mut self.i, n))
            .as_ptr()
            .read()
    }
}

impl<T, const N: usize> Drop for ArrayIter<T, N> {
    fn drop(&mut self) {
        unsafe {
            let slice = self.data.get_unchecked_mut(self.i..);
            let slice = &mut *(slice as *mut [_] as *mut [T]);
            core::ptr::drop_in_place(slice);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ArrayIterator, IntoArrayIterator};

    struct DropCheck(usize);
    impl Drop for DropCheck {
        fn drop(&mut self) {
            println!("drop {}", self.0);
        }
    }

    #[test]
    fn drop_check() {
        let a = [
            DropCheck(0),
            DropCheck(1),
            DropCheck(2),
            DropCheck(3),
        ];

        let result = std::panic::catch_unwind(|| {
            a.into_array_iter()
                .map(|d| {
                    assert!(d.0 < 2);
                    d
                })
                .collect()
        });

        assert!(result.is_err())
    }
}
