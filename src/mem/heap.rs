//! Global kernel heap allocator for arbitrary sized virtual allocations

use core::{alloc::{GlobalAlloc, Layout}, ptr::NonNull};

use linked_list_allocator::LockedHeap;

use crate::mem::PAGE_SIZE;

struct KernelHeap {
    small_alloc: LockedHeap,
}

impl KernelHeap {
    /// Return true if should use free list allocator
    fn is_small(layout: Layout) -> bool {
        layout.size() >= PAGE_SIZE / 2 ||
            layout.align() >= PAGE_SIZE / 2
    }
}

unsafe impl GlobalAlloc for KernelHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if Self::is_small(layout) {
            let mut smalloc = self.small_alloc.lock();

            if let Ok(alloc) = smalloc.allocate_first_fit(layout) {
                alloc.as_ptr() 
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if Self::is_small(layout) {
            unsafe {
                self.small_alloc
                    .lock()
                    .deallocate(
                        NonNull::new_unchecked(ptr), 
                        layout
                    );
            }
        } else {
            todo!()
        }
    }
}

#[global_allocator]
static HEAP: KernelHeap = KernelHeap{ small_alloc: LockedHeap::empty() };
