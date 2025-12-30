#![no_std]
#![no_main]

mod sbi;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain(_hart_id: usize, _dtb: usize) -> ! {

    sbi::debug_console::write_byte('H' as u8).unwrap();
    // Kernel entry point - running in higher half with MMU enabled
    loop {}
}
