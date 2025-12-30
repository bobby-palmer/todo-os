use core::arch::asm;

/// Trap frame saved by trap handler
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
}

/// Trap cause values
#[derive(Debug, Clone, Copy)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    SupervisorSoftware,
    SupervisorTimer,
    SupervisorExternal,
    Unknown(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Exception {
    InstructionMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadMisaligned,
    LoadAccessFault,
    StoreMisaligned,
    StoreAccessFault,
    UserEcall,
    SupervisorEcall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown(usize),
}

impl From<usize> for Trap {
    fn from(scause: usize) -> Self {
        let is_interrupt = (scause >> 63) & 1 == 1;
        let code = scause & 0x7fff_ffff_ffff_ffff;

        if is_interrupt {
            Trap::Interrupt(match code {
                1 => Interrupt::SupervisorSoftware,
                5 => Interrupt::SupervisorTimer,
                9 => Interrupt::SupervisorExternal,
                _ => Interrupt::Unknown(code),
            })
        } else {
            Trap::Exception(match code {
                0 => Exception::InstructionMisaligned,
                1 => Exception::InstructionAccessFault,
                2 => Exception::IllegalInstruction,
                3 => Exception::Breakpoint,
                4 => Exception::LoadMisaligned,
                5 => Exception::LoadAccessFault,
                6 => Exception::StoreMisaligned,
                7 => Exception::StoreAccessFault,
                8 => Exception::UserEcall,
                9 => Exception::SupervisorEcall,
                12 => Exception::InstructionPageFault,
                13 => Exception::LoadPageFault,
                15 => Exception::StorePageFault,
                _ => Exception::Unknown(code),
            })
        }
    }
}

/// Initialize trap handling
pub fn init() {
    unsafe {
        // Set stvec to our trap handler (direct mode)
        asm!(
            "la t0, _trap_entry",
            "csrw stvec, t0",
            options(nomem, nostack)
        );
    }
    crate::println!("trap: initialized");
}

/// Rust trap handler called from assembly
#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(frame: &mut TrapFrame) {
    let scause: usize;
    let stval: usize;
    let sepc: usize;

    unsafe {
        asm!("csrr {}, scause", out(reg) scause, options(nomem, nostack));
        asm!("csrr {}, stval", out(reg) stval, options(nomem, nostack));
        asm!("csrr {}, sepc", out(reg) sepc, options(nomem, nostack));
    }

    let trap = Trap::from(scause);

    match trap {
        Trap::Interrupt(int) => handle_interrupt(int, frame),
        Trap::Exception(exc) => handle_exception(exc, sepc, stval, frame),
    }
}

fn handle_interrupt(int: Interrupt, _frame: &mut TrapFrame) {
    match int {
        Interrupt::SupervisorTimer => {
            crate::println!("trap: timer interrupt");
        }
        Interrupt::SupervisorSoftware => {
            crate::println!("trap: software interrupt");
        }
        Interrupt::SupervisorExternal => {
            crate::println!("trap: external interrupt");
        }
        Interrupt::Unknown(code) => {
            crate::println!("trap: unknown interrupt {}", code);
        }
    }
}

fn handle_exception(exc: Exception, sepc: usize, stval: usize, _frame: &mut TrapFrame) {
    match exc {
        Exception::Breakpoint => {
            crate::println!("trap: breakpoint at {:#x}", sepc);
            // Skip past ebreak instruction
            unsafe {
                asm!("csrr t0, sepc", "addi t0, t0, 2", "csrw sepc, t0", options(nomem, nostack));
            }
        }
        _ => {
            panic!(
                "unhandled exception: {:?}\n  sepc:  {:#x}\n  stval: {:#x}",
                exc, sepc, stval
            );
        }
    }
}
