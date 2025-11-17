#[derive(Debug)]
pub enum SbiError {
    SbiErrFailed,
    SbiErrNotSupported,
    SbiErrInvalidParam,
    SbiErrDenied,
    SbiErrInvalidAddress,
    SbiErrAlreadyAvailable,
    SbiErrAlreadyStarted,
    SbiErrAlreadyStopped,
    SbiErrNoShmem,
    SbiErrInvalidState,
    SbiErrBadRange,
    SbiErrTimeout,
    SbiErrIo
}

#[derive(Default)]
struct SbiArgs {
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
}

/// Make a generic OpenSBI call
fn call(eid: i32, fid: i32, args: SbiArgs) -> Result<isize, SbiError> {
    let mut error_code: isize;
    let mut value: isize;

    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") args.a0,
            in("a1") args.a1,
            in("a2") args.a2,
            in("a3") args.a3,
            in("a4") args.a4,
            in("a5") args.a5,
            in("a6") fid,
            in("a7") eid,

            lateout("a0") error_code,
            lateout("a1") value,
            options(nostack)
        );
    }

    match error_code {
        0 => Ok(value),
        -1 => Err(SbiError::SbiErrFailed),
        -2 => Err(SbiError::SbiErrNotSupported),
        -3 => Err(SbiError::SbiErrInvalidParam),
        -4 => Err(SbiError::SbiErrDenied),
        -5 => Err(SbiError::SbiErrInvalidAddress),
        -6 => Err(SbiError::SbiErrAlreadyAvailable),
        -7 => Err(SbiError::SbiErrAlreadyStarted),
        -8 => Err(SbiError::SbiErrAlreadyStopped),
        -9 => Err(SbiError::SbiErrNoShmem),
        -10 => Err(SbiError::SbiErrInvalidState),
        -11 => Err(SbiError::SbiErrBadRange),
        -12 => Err(SbiError::SbiErrTimeout),
        -13 => Err(SbiError::SbiErrIo),
        _ => unreachable!()
    }
}

pub mod debug_console {
    use super::*;
    const EID: i32 = 0x4442434E;
    // TODO remove this when mapping works
    const OFFSET: usize = 0xffffffc000000000 - 0x80000000;

    pub fn console_write(message: &str) -> Result<usize, SbiError> {
        call(EID, 0x0, SbiArgs{
            a0: message.len(),
            a1: message.as_ptr() as usize - OFFSET,
            ..Default::default()
        }).map(|result| result as usize)
    }
}
