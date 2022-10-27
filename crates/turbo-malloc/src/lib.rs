use std::alloc::{GlobalAlloc, Layout};

pub struct TurboMalloc;

unsafe impl GlobalAlloc for TurboMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        mimalloc::MiMalloc.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        mimalloc::MiMalloc.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        mimalloc::MiMalloc.alloc_zeroed(layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        mimalloc::MiMalloc.realloc(ptr, layout, new_size)
    }
}
