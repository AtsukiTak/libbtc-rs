use std::marker::PhantomData;

use libc::c_void;

use libbtc_sys::{btc_free, btc_true};
use libbtc_sys::vector::{vector_add, vector_free, vector_new, Vector};

/// Abstract type for `libbtc::Vector`.
pub struct BtcVec<T: 'static> {
    inner: *mut Vector, // This ptr's lifetime is 'static.
    t: PhantomData<T>,
}

impl<T: 'static> BtcVec<T> {
    /// Construct new `BtcVec`.
    pub fn new() -> BtcVec<T> {
        unsafe { BtcVec::from_inner_vec(vector_new(0, free_vec_item)) }
    }

    fn inner_ref(&self) -> &'static Vector {
        unsafe {
            self.inner.as_ref() // Option<&'static Vector>
                .unwrap() // &'static Vector
        }
    }

    /// Return `true` if it contains nothing else return `false`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns current number of items.
    pub fn len(&self) -> usize {
        self.inner_ref().len
    }

    /// Returns reference to internal item.
    /// Note that since all internal items are allocated onto heap memory,
    /// returned item's lifetime is 'static.
    pub fn index(&self, idx: usize) -> &'static T {
        if idx < self.len() {
            unsafe {
                let &raw = self.inner_ref().data // *mut *mut c_void
                    .offset(idx as isize) // *mut *mut c_void
                    .as_ref() // Option<&*mut c_void>
                    .unwrap(); // &*mut c_void
                (raw as *mut T).as_ref().unwrap()
            }
        } else {
            panic!("Out of bounds");
        }
    }

    /// Create a new iterator.
    /// `BtcVecIter` iterates allocated pointer to `T`.
    pub fn iter(&self) -> BtcVecIter<T> {
        BtcVecIter { inner: self, n: 0 }
    }

    /// Given `item` must be allocated via `btc_malloc` or `btc_calloc` function.
    ///
    /// # Unsafe
    /// It is unsafe when `item` is not created using `btc_malloc` or `btc_calloc`.
    pub unsafe fn push(&mut self, item: *mut T) {
        self.use_inner_vec(|inner_vec| {
            if vector_add(inner_vec, item as *mut c_void) == 0 {
                panic!("Fail to push");
            }
            (inner_vec, ())
        });
    }

    /// Useful function when you want to access raw `libbtc::Vector`.
    /// You can use it for anything but you must return it inside closure.
    pub fn use_inner_vec<F, U>(&mut self, f: F) -> U
    where
        F: FnOnce(*mut Vector) -> (*mut Vector, U),
    {
        let (inner_vec, item) = f(self.inner);
        self.inner = inner_vec;
        item
    }

    /// Construct new `BtcVec`.
    ///
    /// # Unsafe
    /// This function can't guarantee
    /// - A lifetime of given `Vector` is `'static`.
    /// - Type `T` is valid type for `Vector`.
    pub unsafe fn from_inner_vec(inner: *mut Vector) -> BtcVec<T> {
        BtcVec {
            inner: inner,
            t: PhantomData,
        }
    }
}

// Contained item must be created via `btc_malloc` or `btc_calloc`.
extern "C" fn free_vec_item(raw: *mut c_void) {
    unsafe { btc_free(raw) }
}

/// `Iterator` of `BtcVec`.
/// Note that each iteration item has lifetime 'static.
pub struct BtcVecIter<'a, T: 'static> {
    inner: &'a BtcVec<T>,
    n: usize,
}

impl<'a, T: 'static> Iterator for BtcVecIter<'a, T> {
    type Item = &'static T;

    fn next(&mut self) -> Option<&'static T> {
        let n = self.n;
        if n < self.inner.len() {
            self.n = n + 1;
            Some(self.inner.index(n))
        } else {
            None
        }
    }
}

impl<T: 'static> Drop for BtcVec<T> {
    fn drop(&mut self) {
        unsafe { vector_free(self.inner, btc_true) };
    }
}
