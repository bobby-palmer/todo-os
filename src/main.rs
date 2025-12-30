#![no_std]
#![no_main]

mod print;
mod sbi;
mod trap;

use core::arch::asm;

/// The boot hart (id = 0) jumps here after boot.s
/// with page table, stack, bss setup
#[unsafe(no_mangle)]
pub extern "C" fn kmain(_hart_id: usize, _dtb: usize) -> ! {
    println!("Hello from todo-os!");
    println!("hart_id: {}, dtb: {:#x}", _hart_id, _dtb);

    trap::init();

    // Test breakpoint exception
    println!("Testing breakpoint...");
    unsafe { asm!("ebreak") };
    println!("Returned from breakpoint!");

    loop {}
}

use core::panic::PanicInfo;

/// Global kernel panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("=== Kernel Panic! ===");
    loop {}
}

