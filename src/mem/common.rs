use core::ptr::NonNull;

/// Rust repr for a single 4KB Page
#[repr(align(0x1000), C)]
pub struct Page(pub [u8; 0x1000]);

impl Page {
    pub fn ppn(&self) -> u64 {
        let vaddr = (&raw const self) as usize;
        let paddr = vaddr - VIRTUAL_RAM_START + PHYSICAL_RAM_START;
        (paddr / PAGE_SIZE) as u64
    }

    pub fn from_ppn(ppn: u64) -> NonNull<Page> {
        let paddr = ppn as usize * PAGE_SIZE;
        let vaddr = paddr - PHYSICAL_RAM_START + VIRTUAL_RAM_START;
        unsafe { NonNull::new_unchecked(vaddr as *mut Page) }
    }
}

pub const PAGE_SIZE: usize = 0x1000;
pub const PHYSICAL_RAM_START: usize =        0x80000000;
pub const VIRTUAL_RAM_START: usize = 0xffffffc000000000;
