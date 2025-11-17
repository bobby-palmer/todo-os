/// Rust repr for a single 4KB Page
#[repr(align(0x1000), C)]
pub struct Page([u8; 0x1000]);

pub const PAGE_SIZE: usize = 0x1000;
pub const PHYSICAL_RAM_START: usize =        0x80000000;
pub const VIRTUAL_RAM_START: usize = 0xffffffc000000000;
