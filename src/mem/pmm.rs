//! Global physical Memory Manager (pmm). Used to manage physical ram frames. 
//! Should be wrapped in safety checks before exposing outside of mem module

use core::ptr::NonNull;

use spin::Mutex;

use crate::mem::{PAGE_SIZE, PHYSICAL_RAM_START, VIRTUAL_RAM_START};

/// Repr for a physical page frame
#[repr(align(0x1000), C)]
pub struct Page(pub [u8; 0x1000]);

impl Page {
    /// Get physical page number
    pub fn ppn(&self) -> u64 {
        let vaddr = self as *const Self as usize;
        let offset = vaddr - VIRTUAL_RAM_START;
        ((PHYSICAL_RAM_START + offset) / PAGE_SIZE) as u64
    }

    /// Get frame from physical page number
    pub fn from_ppn(ppn: u64) -> NonNull<Self> {
        let pn_offset = ppn - (PHYSICAL_RAM_START / PAGE_SIZE) as u64;
        let vaddr = VIRTUAL_RAM_START + pn_offset as usize * PAGE_SIZE;
        unsafe {NonNull::new_unchecked(vaddr as *mut Self)}
    }
}

pub fn alloc_page() -> Option<NonNull<Page>> {
    let mut list = FREE_LIST.lock();
    let page = list.head?;
    let node = page.cast::<Link>();

    unsafe { list.head = node.read(); }
    list.length -= 1;

    Some(page)
}

pub fn free_page(page: NonNull<Page>) {
    let mut list = FREE_LIST.lock();
    let node = page.cast::<Link>();

    unsafe { node.write(list.head); }
    list.head = Some(page);
    list.length += 1;
}

/// Number of free frames managed by the pmm (for debug)
pub fn count_free() -> usize {
    FREE_LIST.lock().length
}

type Link = Option<NonNull<Page>>;

struct FreeList {
    length: usize,
    head: Link,
}

unsafe impl Send for FreeList {}

static FREE_LIST: Mutex<FreeList> = Mutex::new(FreeList{
    length: 0,
    head: None,
});
