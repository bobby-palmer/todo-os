#![no_std]
#![no_main]

use core::cmp::max;

use crate::memory::PhysicalAddress;

mod sbi;
mod console;
mod memory;
mod exception;
mod riscv;

unsafe extern "C" {
    static mut _sbss: u8;
    static mut _ebss: u8;
}

/// Entry point for the boot hart after running asm/boot.S
#[unsafe(no_mangle)]
extern "C" fn _kmain(_hart_id: u64, fdt_ptr: *const u8) -> ! {
    init_bss();

    let fdt = unsafe {fdt::Fdt::from_ptr(fdt_ptr).unwrap()};

    let mut highest_reserved = PhysicalAddress::null();

    // System reserved memory info
    let mut reservations = fdt.memory_reservations();
    while let Some(region) = reservations.next() {
        let reservation_start: PhysicalAddress = 
            (region.address() as usize).into();
        let reservation_end = reservation_start + region.size();

        highest_reserved = max(highest_reserved, reservation_end);
    }

    // Open sbi reserved memory info
    if let Some(reservation_node) = fdt.find_node("/reserved-memory") {
        let mut children = reservation_node.children();
        while let Some(child) = children.next() {
            let mut regions = child.reg().unwrap();
            while let Some(region) = regions.next() {
                let reservation_start: PhysicalAddress = 
                    (region.starting_address as usize).into();
                let reservation_end = reservation_start + region.size.unwrap();

                highest_reserved = max(highest_reserved, reservation_end);
            }
        }
    }

    println!("Highest reserved addr: {:?}", highest_reserved);

    // Get ram info (only supports one ram region)
    let memory_info = fdt.memory();
    let free_ram_info = memory_info.regions().next().unwrap();

    let ram_start: PhysicalAddress = 
        (free_ram_info.starting_address as usize).into();
    let ram_end = ram_start + free_ram_info.size.unwrap();

    println!("RAM: start {:?}, end {:?}", ram_start, ram_end);

    memory::init(highest_reserved, ram_end);

    // END
    println!("Hello from kmain");
    loop {}
}

/// Global panic handler
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("!!! Kernel Panic !!!");
    println!("{}", info.message());
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
