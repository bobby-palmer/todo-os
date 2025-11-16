mod heap;
mod page_list;

/// Struct representation of a ram page
#[repr(C)]
#[repr(align(0x1000))]
struct Page {
    bytes: [u8; 0x1000]
}
