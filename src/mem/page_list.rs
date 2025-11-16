//! Page frame free list to use internally within memory module

use core::ptr::NonNull;

use spin::Mutex;

use crate::mem::Page;

pub(super) fn alloc() -> Option<NonNull<Page>> {
    FREE_LIST.lock().alloc()
}

pub(super) fn free(page: NonNull<Page>) {
    FREE_LIST.lock().free(page);
}

static FREE_LIST: Mutex<FreeList> = Mutex::new(FreeList::new());

type Node = Option<NonNull<Page>>;

struct FreeList {
    length: usize,
    head: Node,
}

impl FreeList {
    const fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    fn alloc(&mut self) -> Option<NonNull<Page>> {
        unsafe {
            let page = self.head?;
            let next = page.cast::<Node>();
            self.head = next.read();
            self.length -= 1;
            Some(page)
        }
    }

    fn free(&mut self, page: NonNull<Page>) {
        unsafe {
            let next = page.cast::<Node>();

            next.write(self.head);
            self.head = Some(page);
            self.length += 1;
        }
    }
}

unsafe impl Send for FreeList {}
