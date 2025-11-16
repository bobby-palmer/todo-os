//! Internal single page frame allocator. 

use core::ptr::NonNull;

use spin::Mutex;

use crate::mem::address::Page;

pub(super) unsafe fn alloc() -> Option<NonNull<Page>> {
    let mut list = FREE_LIST.lock();
    let node = list.head.0?;

    unsafe { list.head = node.read(); }
    list.length -= 1;

    Some(node.cast())
}

pub(super) unsafe fn free(page: NonNull<Page>) {
    let mut list = FREE_LIST.lock();
    let node = page.cast::<Node>();

    unsafe { node.write(list.head); }
    list.head.0 = Some(node);
    list.length += 1;
}

#[derive(Clone, Copy)]
struct Node(Option<NonNull<Node>>);

struct FreeList {
    length: usize,
    head: Node,
}

unsafe impl Send for FreeList {}

static FREE_LIST: Mutex<FreeList> = Mutex::new(FreeList{
    length: 0,
    head: Node(None)
});
