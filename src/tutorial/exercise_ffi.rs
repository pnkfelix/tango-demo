extern crate libc;

#[cfg(test)]
use std::mem;
#[cfg(test)]
use self::libc::{c_void, c_int};

#[cfg(test)]
extern {
    fn qsort(base: *mut c_void,
             nmemb: libc::size_t,
             size: libc::size_t,
             compar: fn (*const c_void, *const c_void) -> c_int);
}

#[test]
fn demo_qsort() {
    fn u8_cmp(a: *const c_void, b: *const c_void) -> c_int {
        let a = a as *const u8;
        let b = b as *const u8;

//@ A quick interruption here to point out that this variant
//@ of the computation overflows, and Rust signals such.

        // (unsafe { *a as i8 - *b as i8 }) as c_int

//@ Here is a way to avoid the overflow:

        (unsafe { *a as i16 - *b as i16 }) as c_int

//@ Compare with:

        // (unsafe { (*a as i8).wrapping_sub(*b as i8) }) as c_int


    }
    let mut array_a = [255, 128, 1u8, 5, 2, 14];
    let mut array_b: [u8; 11] = *b"Hello World";

    unsafe {
        println!(" pre(a): {:?}", array_a);
        let len = array_a.len();
        qsort(array_a.as_mut_ptr() as *mut c_void, len as libc::size_t, mem::size_of::<u8>() as libc::size_t, u8_cmp);
        println!("post(a): {:?}", array_a);

        println!(" pre(b): {:?}", array_b);
        let len = array_b.len();
        qsort(array_b.as_mut_ptr() as *mut c_void, len as libc::size_t, mem::size_of::<u8>() as libc::size_t, u8_cmp);
        println!("post(b): {:?}", array_b);
    }
}
