//! Kernel ram management module

mod addr;
mod page_table;
mod pmm;
mod vmm;

use fdt::Fdt;

pub const PAGE_SIZE: usize = 0x1000;
pub const PHYSICAL_RAM_START: usize =        0x80000000;
pub const VIRTUAL_RAM_START: usize = 0xffffffc000000000;

/// One time boot function
pub fn init(fdt: &Fdt) {
    let ram = fdt.memory().regions().next().unwrap();
}
