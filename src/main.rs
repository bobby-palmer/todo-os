#![no_std]
#![no_main]

mod sbi;
mod mem;
mod console;
mod panic;

/// Entry point to kernel boot strap after boot.S
#[unsafe(no_mangle)]
extern "C" fn kmain(_hart_id: usize, fdt_ptr: *const u8) -> ! {

    bss_init();

    let fdt = unsafe {fdt::Fdt::from_ptr(fdt_ptr).unwrap()};

    mem::init(&fdt);

    panic!("END OF KERNEL");
}


/// Zero out the bss
fn bss_init() {
    unsafe {
        let bss_start = &raw mut _sbss;
        let bss_end = &raw const _ebss;
        core::ptr::write_bytes(
            bss_start, 0, bss_end as usize - bss_start as usize);

    }
}

// Linker symbols
unsafe extern "C" {
    static mut _sbss: u8;
    static mut _ebss: u8;
}
