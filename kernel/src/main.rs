#![no_std]
#![no_main]

mod sbi;
mod console;
mod memory;

unsafe extern "C" {
    static mut _sbss: u8;
    static mut _ebss: u8;
}

/// Entry point for the boot hart after running asm/boot.S
#[unsafe(no_mangle)]
extern "C" fn _kmain(_hart_id: u64, fdt_ptr: *const u8) -> ! {
    init_bss();

    let fdt = unsafe {fdt::Fdt::from_ptr(fdt_ptr).unwrap()};
    println!("fdt is {} bytes", fdt.total_size());
    println!("Hello from kmain");
    loop {}
}

/// Global panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Zero out the bss so that global variables are ok
fn init_bss() {
    unsafe {
        let start = &raw mut _sbss;
        let end = &raw const _ebss;
        let length = end as usize - start as usize;
        core::ptr::write_bytes(start, 0, length);
    }
}

#[macro_export]
macro_rules! print {
    ($fmt:literal $($arg:tt)*) => ({
        use core::fmt::Write;
        let mut writer = console::DebugWriter;
        let _ = write!(writer, $fmt $($arg)*);
    })
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:literal $($arg:tt)*) => ({
        // Call the print! macro with all arguments
        $crate::print!($fmt $($arg)*);
        // Then print the newline
        $crate::print!("\n"); 
    });
}
