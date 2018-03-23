use std::marker::PhantomData;

use libc::c_void;

use libbtc_sys::{btc_free, vector_add, vector_free, vector_new, Vector};

type Should<T> = Option<T>;

// Contained item must be created via `btc_malloc` or `btc_calloc`.
extern "C" fn free_vec_item(raw: *mut c_void) {
    unsafe { btc_free(raw) }
}

/// Abstract type for `libbtc::Vector`.
pub struct BtcVec<T: 'static> {
    vec: Should<*mut Vector>, // This ptr's lifetime must be 'static.
    t: PhantomData<T>,
}

impl<T: 'static> BtcVec<T> {
    /// Construct new `BtcVec`.
    pub fn new() -> BtcVec<T> {
        unsafe { BtcVec::from_inner_vec(vector_new(0, free_vec_item)) }
    }

    fn inner_ref(&self) -> &Vector {
        unsafe {
            self.vec.as_ref() // Option<&*mut Vector>
                .unwrap() // &*mut Vector
                .as_ref() // Option<&Vector>
                .unwrap() // &Vector
        }
    }

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

    pub fn iter(&self) -> BtcVecIter<T> {
        BtcVecIter { inner: self, n: 0 }
    }

    /// Given `item` must be allocated via `btc_malloc` or `btc_calloc` function.
    pub fn push(&mut self, item: *mut T) {
        self.use_inner_vec(|inner_vec| {
            unsafe {
                if vector_add(inner_vec, item as *mut c_void) == 0 {
                    panic!("Fail to push");
                }
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
        let inner_vec = self.vec.take().unwrap();
        let (inner_vec, item) = f(inner_vec);
        self.vec = Some(inner_vec);
        item
    }

    /// Construct new `BtcVec`.
    ///
    /// # Unsafe
    /// This function can't guarantee
    /// - A lifetime of given `Vector` is `'static`.
    /// - Type `T` is valid type for `Vector`.
    pub unsafe fn from_inner_vec(vec: *mut Vector) -> BtcVec<T> {
        BtcVec {
            vec: Some(vec),
            t: PhantomData,
        }
    }
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
        let inner_vec = self.vec.take().unwrap();
        unsafe { vector_free(inner_vec, true as u8) };
    }
}
