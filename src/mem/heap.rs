//! Global heap allocator that can be accessed via core::alloc

use core::alloc::GlobalAlloc;

/// Global kernel memory allocator. Allocates in kernel space
struct KernelHeap;

impl KernelHeap {
    const fn new() -> Self {
        Self
    }
}

unsafe impl GlobalAlloc for KernelHeap {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        todo!()
    }
}

#[global_allocator]
static GLOBAL_HEAP: KernelHeap = KernelHeap::new();
