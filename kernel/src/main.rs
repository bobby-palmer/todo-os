#![no_std]
#![no_main]

/// Entry point after setting page table to run in the higher half. 
/// This function should never return!
#[unsafe(no_mangle)]
extern "C" fn _kmain(_hart_id: u64, _fdt_ptr: u64) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
