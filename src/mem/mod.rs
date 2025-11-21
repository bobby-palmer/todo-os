pub const PAGE_SIZE: usize = 0x1000;
pub const PHYSICAL_RAM_START: usize =        0x80000000;
pub const VIRTUAL_RAM_START: usize = 0xffffffc000000000;

// pub mod page_table;
mod pmm;
mod vmm;
mod page_table;

use fdt::Fdt;

unsafe extern "C" {
    static mut _kend: u8;
}

/// One time initialization for the boot hart to initialize free ram
pub fn init(fdt: &Fdt) {
    // Initialize pmm with free frames
    let ram = fdt.memory().regions().next().unwrap();

    let start_addr = ram.starting_address as usize;
    let end_addr = start_addr + ram.size.unwrap();

    let start_ppn = (start_addr + PAGE_SIZE - 1) / PAGE_SIZE;
    let end_ppn =  end_addr / PAGE_SIZE;

    // TODO make this more thorough
    let is_reserved = |ppn: usize| {
        let phys_kern_end = &raw const _kend as usize
            - VIRTUAL_RAM_START + PHYSICAL_RAM_START;
        
        if ppn * PAGE_SIZE < phys_kern_end {
            true
        } else {
            false
        }
    };

    for ppn in start_ppn..end_ppn {
        if !is_reserved(ppn) {
            pmm::free_page(pmm::Page::from_ppn(ppn as u64));
        }
    }
}
