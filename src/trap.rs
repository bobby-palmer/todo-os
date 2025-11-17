//! Trap handler for user/supervisor exception (ie syscalls)

use core::{arch::asm, ptr::NonNull};

/// Set the trap entry function on this hart, must be called on all
pub fn init_hart() {
    unsafe {
        asm!(
            "la t0, _trapentry",
            "csrw stvec, t0",
        );
    }
}

#[unsafe(no_mangle)]
extern "C" fn handle_trap(_trap_frame: NonNull<TrapFrame>) {
    todo!()
}

/// State of the cpu when trap occured, keep in sync with asm/trap.S
struct TrapFrame {
    /// GPRs
    regs: [usize; 31],
}
