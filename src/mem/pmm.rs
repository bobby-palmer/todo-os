//! Global physical memory manager for allocating page frames.
//! Currently just a free list

use core::ptr::NonNull;

use spin::Mutex;

use crate::mem::{PAGE_SIZE, PHYSICAL_RAM_START, VIRTUAL_RAM_START};

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
        self.paddr() - PHYSICAL_RAM_START + VIRTUAL_RAM_START
    }

    /// Get virtual pointer to page casted as an object
    pub fn as_ptr<T>(&self) -> NonNull<T> {
        NonNull::new(self.vaddr() as *mut T).unwrap()
    }
}

pub struct OutOfMemory;

pub fn alloc_page() -> Result<Page, OutOfMemory> {
    let mut list = FREE_LIST.lock();
    let page = list.head.ok_or(OutOfMemory)?;

    list.length -= 1;
    unsafe { 
        list.head = page.as_ptr::<Option<Page>>().read(); 
    }

    Ok(page)
}

pub fn free_page(page: Page) {
    let mut list = FREE_LIST.lock();

    list.length += 1;
    unsafe { 
        page.as_ptr::<Option<Page>>().write(list.head);
    }
    list.head = Some(page);
}

struct FreeList {
    length: usize,
    head: Option<Page>,
}

static FREE_LIST: Mutex<FreeList> = Mutex::new(FreeList{
    length: 0,
    head: None,
});
