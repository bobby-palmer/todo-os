pub const PAGE_SIZE: usize = 0x1000;
pub const KERNEL_SPACE_START: usize = 0xffff800000000000;

mod address;
pub use address::*;

pub mod pfa;

pub fn init(start_free_ram: PhysicalAddress, end_free_ram: PhysicalAddress) {
    pfa::init(start_free_ram, end_free_ram);
}
