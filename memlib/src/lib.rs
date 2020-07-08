#![no_std]

use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering::SeqCst};

use libc;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

// for allocated memory counting

pub fn get_allocated() -> usize {
    ALLOCATED.load(SeqCst)
}

// see for an interface: https://doc.rust-lang.org/src/alloc/alloc.rs.html#15

#[no_mangle]
pub unsafe extern "C" fn __rust_alloc(size: usize, _align: usize) -> *mut u8 {
    let ptr = crate::libc::malloc(size) as *mut u8;
    if !ptr.is_null() {
        crate::ALLOCATED.fetch_add(size, crate::SeqCst);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn __rust_dealloc(ptr: *mut u8, _old_size: usize, _align: usize) {
    crate::libc::free(ptr as *mut crate::c_void);
    crate::ALLOCATED.fetch_sub(_old_size, crate::SeqCst);
}

#[no_mangle]
pub unsafe extern "C" fn __rust_realloc(
    ptr: *mut u8,
    old_size: usize,
    _align: usize,
    new_size: usize,
) -> *mut u8 {
    let ret = crate::libc::realloc(ptr as *mut crate::c_void, new_size) as *mut u8;
    if !ret.is_null() {
        crate::ALLOCATED.fetch_sub(old_size, crate::SeqCst);
        crate::ALLOCATED.fetch_add(new_size, crate::SeqCst);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    let ptr = __rust_alloc(size, align);
    if !ptr.is_null() {
        crate::ptr::write_bytes(ptr, 0, size);
    }
    ptr
}
