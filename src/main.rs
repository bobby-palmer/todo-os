#![no_std]
#![no_main]

/// Entry point to kernel bootstrap after boot.S
#[unsafe(no_mangle)]
extern "C" fn kmain(_hart_id: usize, fdt_ptr: *const u8) -> ! {
    bss_init();
    let _fdt = unsafe {fdt::Fdt::from_ptr(fdt_ptr).unwrap()};
    loop {}
}

/// Set the global panic handler for kernel
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
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
