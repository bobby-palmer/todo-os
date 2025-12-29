//! Open-SBI wrappers for machine level ecall.
//! See https://www.scs.stanford.edu/~zyedidia/docs/riscv/riscv-sbi.pdf

pub const SbiError = error{
    FAILED,
    NOT_SUPPORTED,
    INVALID_PARAM,
    DENIED,
    INVALID_ADDRESS,
    ALREADY_AVAILABLE,
    ALREADY_STARTED,
    ALREADY_STOPPED,
};

// Generic ecall wrapper
fn ecall(eid: i32, fid: i32, args: []const usize) SbiError!isize {
    var a0: usize = if (args.len > 0) args[0] else 0;
    var a1: usize = if (args.len > 1) args[1] else 0;
    const a2: usize = if (args.len > 2) args[2] else 0;
    const a3: usize = if (args.len > 3) args[3] else 0;
    const a4: usize = if (args.len > 4) args[4] else 0;
    const a5: usize = if (args.len > 5) args[5] else 0;
    const a6: usize = @bitCast(@as(isize, fid));
    const a7: usize = @bitCast(@as(isize, eid));

    asm volatile ("ecall"
        : [a0] "={a0}" (a0),
          [a1] "={a1}" (a1),
        : [in_a0] "{a0}" (a0),
          [in_a1] "{a1}" (a1),
          [in_a2] "{a2}" (a2),
          [in_a3] "{a3}" (a3),
          [in_a4] "{a4}" (a4),
          [in_a5] "{a5}" (a5),
          [in_a6] "{a6}" (a6),
          [in_a7] "{a7}" (a7),
        : .{ .memory = true }
    );

    // a0 contains error code, a1 contains return value
    const err: isize = @bitCast(a0);
    const val: isize = @bitCast(a1);

    return switch (err) {
        0 => val,
        -1 => SbiError.FAILED,
        -2 => SbiError.NOT_SUPPORTED,
        -3 => SbiError.INVALID_PARAM,
        -4 => SbiError.DENIED,
        -5 => SbiError.INVALID_ADDRESS,
        -6 => SbiError.ALREADY_AVAILABLE,
        -7 => SbiError.ALREADY_STARTED,
        -8 => SbiError.ALREADY_STOPPED,
        else => unreachable,
    };
}

/// Legacy open-SBI calls, deprecate eventually. Function ID doesn't matter for these.
pub const Legacy = struct {

    /// Write data present in ch to debug console. Unlike
    /// sbi_console_getchar(), this SBI call will block if there remain any
    /// pending characters to be transmitted or if the receiving terminal is
    /// not yet ready to receive the byte. However, if the console doesn’t
    /// exist at all, then the character is thrown away. This SBI call returns
    /// 0 upon success or an implementation specific negative error code.
    pub fn console_putchar(ch: u8) SbiError!void {
        _ = try ecall(0x01, 0x00, &.{ @intCast(ch) });
    }
};
