#![no_std]
#![no_main]

mod print;
mod sbi;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain(_hart_id: usize, _dtb: usize) -> ! {
    println!("Hello from todo-os!");
    println!("hart_id: {}, dtb: {:#x}", _hart_id, _dtb);
    loop {}
}
