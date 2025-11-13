use core::{arch::asm, ptr::NonNull};

use crate::console;
use crate::println;

unsafe extern "C" {
    fn _trap_entry();
}

pub fn init() {
    unsafe {
        asm!(
            "csrw stvec, {0}",
            in(reg) _trap_entry as usize,
            options(nostack, nomem)
        )
    } 
}

#[unsafe(no_mangle)]
pub extern "C" fn handle_trap(_tf: NonNull<TrapFrame>) {
    println!("Exception");
    loop {}
}

#[repr(C)]
pub struct TrapFrame {
    pub regs: [usize; 31],
}
