use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub use memlib;

pub fn get_counter() -> usize {
    COUNTER.load(SeqCst)
}

pub fn add_counter(x: usize) {
    COUNTER.fetch_add(x, crate::SeqCst);
}

pub fn sub_counter(x: usize) {
    COUNTER.fetch_sub(x, crate::SeqCst);
}

pub fn made_alloc() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(4);
    v.push(8);
    v.push(16);
    v.push(32);
    v.push(64);
    v.push(128);
    v.push(256);
    v
}
