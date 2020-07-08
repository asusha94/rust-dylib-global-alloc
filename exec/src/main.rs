use dummy_dylib;
// use memlib;

use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        dummy_dylib::add_counter(layout.size());
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        dummy_dylib::sub_counter(layout.size());
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

fn main() {
    println!("dummy_dylib::get_counter(): {}", dummy_dylib::get_counter());
    println!(
        "memlib::get_allocated(): {}",
        dummy_dylib::memlib::get_allocated()
    );

    let mut v = dummy_dylib::made_alloc();

    println!("dummy_dylib::get_counter(): {}", dummy_dylib::get_counter());
    println!(
        "memlib::get_allocated(): {}",
        dummy_dylib::memlib::get_allocated()
    );

    for _ in 0..16 {
        v.push(42);
    }

    println!("dummy_dylib::get_counter(): {}", dummy_dylib::get_counter());
    println!(
        "memlib::get_allocated(): {}",
        dummy_dylib::memlib::get_allocated()
    );
}
