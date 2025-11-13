use core::{arch::asm, ptr::NonNull};

use crate::console;
use crate::println;

unsafe extern "C" {
    fn _trap_entry();
}

/// Set the trap function entry point for this cpu
pub fn init() {
    unsafe {
        asm!(
            "csrw stvec, {0}",
            in(reg) _trap_entry as usize,
            options(nostack, nomem)
        )
    } 
}

/// Supervisor level trap dispatcher
#[unsafe(no_mangle)]
pub extern "C" fn handle_trap(_tf: NonNull<TrapFrame>) {
    println!("Exception");
    loop {}
}

/// Saved state of CPU, keep this in sync with trap.S
#[repr(C)]
pub struct TrapFrame {
    pub regs: [usize; 31],
}
