use std::marker::PhantomData;

use libc::c_void;

use bitcoinrs_sys::{vector_add, vector_new, Vector};

type Should<T> = Option<T>;

/// Abstract type for `libbtc::Vector`.
pub struct BtcVec<T> {
    vec: Should<*mut Vector>, // This ptr's lifetime must be 'static.
    t: PhantomData<T>,
}

impl<T> BtcVec<T> {
    pub fn new() -> BtcVec<T> {
        let nothing_drop = |v| println!("Not drop {:?}", v);
        unsafe { BtcVec::from_inner_vec(vector_new(0, nothing_drop)) }
    }

    fn inner_ref(&self) -> &Vector {
        unsafe {
            self.vec.as_ref() // Option<&*mut Vector>
                .unwrap() // &*mut Vector
                .as_ref() // Option<&Vector>
                .unwrap() // &Vector
        }
    }

    pub fn len(&self) -> usize {
        self.inner_ref().len
    }

    pub fn index(&self, idx: usize) -> &T {
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

pub struct BtcVecIter<'a, T: 'a> {
    inner: &'a BtcVec<T>,
    n: usize,
}

impl<'a, T: 'a> Iterator for BtcVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let n = self.n;
        if n < self.inner.len() {
            self.n = n + 1;
            Some(self.inner.index(n))
        } else {
            None
        }
    }
}
