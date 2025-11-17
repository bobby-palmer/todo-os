/// Rust repr for a single 4KB Page
#[repr(align(0x1000), C)]
pub struct Page([u8; 0x1000]);
