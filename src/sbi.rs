//! Wrappers around OpenSBI machine mode ecall.
//! See https://github.com/riscv-non-isa/riscv-sbi-doc.

#[derive(Debug)]
pub enum Error {
    Failed,
    NotSupported,
    InvalidParam,
    Denied,
    InvalidAddress,
    AlreadyAvailible,
    AlreadyStarted,
    AlreadyStopped,
    NoShmem,
    InvalidState,
    BadRange,
    Timeout,
    Io,
    DeniedLocked,
}

fn ecall(eid: i32, fid: i32, args: &[usize]) -> Result<isize, Error> {
    let (error, value): (isize, isize);

    let a0 = args.get(0).copied().unwrap_or(0);
    let a1 = args.get(1).copied().unwrap_or(0);
    let a2 = args.get(2).copied().unwrap_or(0);
    let a3 = args.get(3).copied().unwrap_or(0);
    let a4 = args.get(4).copied().unwrap_or(0);
    let a5 = args.get(5).copied().unwrap_or(0);

    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") a0,
            in("a1") a1,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") error,
            lateout("a1") value,
        );
    }

    match error {
        0 => Ok(value),
        -1 => Err(Error::Failed),
        -2 => Err(Error::NotSupported),
        -3 => Err(Error::InvalidParam),
        -4 => Err(Error::Denied),
        -5 => Err(Error::InvalidAddress),
        -6 => Err(Error::AlreadyAvailible),
        -7 => Err(Error::AlreadyStarted),
        -8 => Err(Error::AlreadyStopped),
        -9 => Err(Error::NoShmem),
        -10 => Err(Error::InvalidState),
        -11 => Err(Error::BadRange),
        -12 => Err(Error::Timeout),
        -13 => Err(Error::Io),
        -14 => Err(Error::DeniedLocked),
        _ => unreachable!("Invalid sbi error code!"),
    }
}

pub mod debug_console {
    const EID: i32 = 0x4442434E;

    pub fn write_byte(byte: u8) -> Result<(), super::Error> {
        super::ecall(EID, 2, &[byte.into()])
            .map(|_| ())
    }
}
