//! Internal page frame allocator for the memory module. will be wrapped to 
//! include ownership guarantees for external callers

use spin::Mutex;

use crate::{constants::PAGE_SIZE, mem::address::PhysicalAddress};

/// Allocate a single page frame
pub(super) fn alloc() -> Option<PhysicalAddress> {
    let mut list = FREE_LIST.lock();

    let node = list.head?;
    unsafe { list.head = *node.to_virtual().as_ptr(); }
    list.length -= 1;

    Some(node)
}

/// Free a single page frame (should be called on frames allocated from "alloc")
pub(super) fn free(frame: PhysicalAddress) {
    assert!(frame.is_aligned(PAGE_SIZE.into()));

    let mut list = FREE_LIST.lock();
    
    unsafe { *frame.to_virtual().as_mut_ptr::<Link>() = list.head; }
    list.head = Some(frame);
    list.length += 1;
}

type Link = Option<PhysicalAddress>;

struct List {
    length: usize,
    head: Link,
}

impl List {
    const fn new() -> Self {
        Self {
            length: 0,
            head: None,
        }
    }
}

static FREE_LIST: Mutex<List> = Mutex::new(List::new());
