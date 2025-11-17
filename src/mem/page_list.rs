//! An intrusive linked list for storing pages

use core::ptr::NonNull;

use crate::mem::common::Page;

type Node = Option<NonNull<Page>>;

pub struct PageList {
    length: usize,
    head: Node,
}

/// To allow mutex wrapping
unsafe impl Send for PageList {}

impl PageList {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }

    pub fn prepend(&mut self, page: NonNull<Page>) {
        let node = page.cast::<Node>();

        unsafe {node.write(self.head); }
        self.head = Some(page);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<NonNull<Page>> {
        let page = self.head?;
        let node = page.cast::<Node>();

        unsafe { self.head = node.read(); }
        self.length -= 1;

        Some(page)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}
