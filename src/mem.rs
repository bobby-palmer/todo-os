pub mod address;
mod pfa;
mod heap;

use fdt::Fdt;

use crate::{constants::{PAGE_SIZE, VIRTUAL_OFFSET}, 
    mem::address::{Alignment, PhysicalAddress}};

unsafe extern "C" {
    static mut _kend: u8;
}

/// Called once at boot to initialize the availible ram.
pub fn init(fdt: &Fdt, fdt_start: PhysicalAddress) {

    // 1) Initialize free ram

    let kend_virtual: *const u8 = &raw const _kend;
    // This is kinda hacky but we need to offset to get the physicall address 
    // of this later replace with a page table translation!
    let kend = PhysicalAddress::new(kend_virtual as usize - VIRTUAL_OFFSET);

    let fdt_end = fdt_start + fdt.total_size();

    let is_reserved = |addr: PhysicalAddress| {
        if addr <= kend {
            true
        } else if fdt_start <= addr + PAGE_SIZE &&  addr <= fdt_end {
            true
        } else {
            false
        }
    };

    let ram = fdt.memory().regions().next().unwrap();

    let start_addr = PhysicalAddress::new(ram.starting_address as usize);
    let end_addr = start_addr + ram.size.unwrap();

    let mut current_addr = start_addr.align_up(Alignment::new(PAGE_SIZE));
    let end_addr = end_addr.align_down(Alignment::new(PAGE_SIZE));

    while current_addr < end_addr {
        if !is_reserved(current_addr) {
            pfa::free(current_addr);
        }

        current_addr += PAGE_SIZE;
    }
}
