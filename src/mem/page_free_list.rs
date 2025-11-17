//! Internal module for storing the free physical pages as a free list.
//! Be careful when calling this, as it has almost zero safety guarantees.

use core::ptr::NonNull;

use spin::Mutex;

use crate::mem::common::Page;

pub fn alloc() -> Option<NonNull<Page>> {
    let mut list = FREE_LIST.lock();

    let page = list.next?;
    let next = page.cast::<NextPtr>();

    unsafe { list.next = next.read(); }
    list.length -= 1;

    Some(page)
}

pub fn free(page: NonNull<Page>) {
    let mut list = FREE_LIST.lock();

    let as_next = page.cast::<NextPtr>();

    unsafe { as_next.write(list.next); }
    list.next = Some(page);
    list.length += 1;
}

/// Return the number of free pages in the list
pub fn count() -> usize {
    FREE_LIST.lock().length
}

type NextPtr = Option<NonNull<Page>>;

struct FreeList {
    length: usize,
    next: NextPtr,
}

unsafe impl Send for FreeList {}

static FREE_LIST: Mutex<FreeList> = Mutex::new(
    FreeList{
        length: 0,
        next: None,
    });
