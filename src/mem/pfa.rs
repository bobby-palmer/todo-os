//! Page frame allocator (pfa). Allocates singular physical page frames

use crate::mem::address::PhysicalAddress;

pub struct PageFrame(PhysicalAddress);

impl PageFrame {

}

pub fn alloc() -> Option<PageFrame> {
    todo!()
}

pub fn free(frame: PageFrame) {
    todo!()
}
