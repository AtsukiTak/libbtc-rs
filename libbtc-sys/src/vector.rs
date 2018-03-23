use libc::{c_void, size_t};

use btc_bool;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vector {
    pub data: *mut *mut c_void, /* array of pointers */
    pub len: size_t,            /* array element count */
    pub alloc: size_t,          /* allocated array elements */

    elem_free_f: extern "C" fn(*mut c_void),
}

#[link(name = "btc", kind = "static")]
extern "C" {
    /* Bindings of vector.h */
    pub fn vector_new(res: size_t, free_f: extern "C" fn(*mut c_void)) -> *mut Vector;
    pub fn vector_free(vec: *mut Vector, free_array: btc_bool);
    pub fn vector_add(vec: *mut Vector, data: *mut c_void) -> btc_bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector() {
        unsafe {
            let vec = vector_new(10, |_v| ());
            let answer: *mut usize = &mut 42;
            let data = ::std::mem::transmute::<*mut usize, *mut c_void>(answer);
            assert!(vector_add(vec, data) == 1);

            let raw_data = vec.as_ref().unwrap().data.as_ref().unwrap()[0];
            assert_eq!(
                ::std::mem::transmute::<*mut c_void, *mut usize>(raw_data)
                    .as_ref()
                    .unwrap(),
                &42
            );
        };
    }
}
