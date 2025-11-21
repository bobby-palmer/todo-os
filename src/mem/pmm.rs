//! Global physical memory manager for allocating page frames

use core::ptr::NonNull;

use crate::mem::PAGE_SIZE;

/// Repr for a physical page pointer
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Page(u64);

impl Page {

    /// Create pointer object from physical page number
    pub fn new(ppn: u64) -> Self {
        Self(ppn)
    }

    /// Return physical page number
    pub fn ppn(&self) -> u64 {
        self.0
    }

    /// Physical address of this page
    pub fn paddr(&self) -> usize {
        self.0 as usize * PAGE_SIZE
    }

    /// A virtual mapping to this page (in the kernel linear map)
    pub fn vaddr(&self) -> usize {
        todo!()
    }

    /// Get virtual pointer to page casted as an object
    pub fn as_ptr<T>(&self) -> NonNull<T> {
        todo!()
    }
}

/// Called once at boot to init free physical page tracking
pub fn init(free_physical_start: usize, free_physical_end: usize) {
    todo!()
}

pub fn alloc(num_pages: usize) -> Option<Page> {
    todo!()
}

pub fn free(base: Page, num_pages: usize) {
    todo!()
}

pub fn alloc_page() -> Option<Page> {
    alloc(1)
}

pub fn free_page(page: Page) {
    free(page, 1)
}
