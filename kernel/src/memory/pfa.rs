//! Page Frame Allocator (pfa). allocate singular physical pages

use spin::Mutex;
use crate::console;

use crate::{memory::{PhysicalAddress, VirtualAddress, PAGE_SIZE}, println};

/// A single RAII physical page that frees itself on drop
pub struct PageFrame(PhysicalAddress);

impl Drop for PageFrame {
    fn drop(&mut self) {
        free(self.0);
    }
}

pub(super) fn init(start_region: PhysicalAddress, end_region: PhysicalAddress) {
    println!("init");
    let aligned_start = start_region.align_up(PAGE_SIZE.into());
    let aligned_end = end_region.align_down(PAGE_SIZE.into());

    let mut page_start = aligned_start;

    while page_start < aligned_end {
        println!("{:?}", page_start);
        free(page_start);
        page_start += PAGE_SIZE;
    }

    println!("end");
}

pub fn alloc() -> Option<PageFrame> {
    let mut list = FREE_LIST.lock();

    if let Some(paddr) = list.list {
        let vaddr: VirtualAddress = paddr.into(); 
        let ptr = vaddr.as_ptr::<Node>();
        unsafe { list.list = *ptr; }
        list.length -= 1;

        Some(PageFrame(paddr))
    } else {
        None
    }
}

/// Return the number of free pages in the pfa
pub fn free_page_count() -> usize {
    let list = FREE_LIST.lock();
    list.length
}

fn free(base_addr: PhysicalAddress) {
    println!("Free");
    let mut list = FREE_LIST.lock();
    println!("Locked");
    let vaddr: VirtualAddress = base_addr.into();
    let ptr = vaddr.as_mut_ptr::<Node>();
    unsafe { *ptr = list.list; }
    list.list = Some(base_addr);
    list.length += 1;
}

type Node = Option<PhysicalAddress>;

struct FreeList {
    length: usize,
    list: Node,
}

impl FreeList {
    const fn new() -> Self {
        Self {
            length: 0,
            list: None,
        }
    }
}

static FREE_LIST: Mutex<FreeList> = Mutex::new(FreeList::new());
