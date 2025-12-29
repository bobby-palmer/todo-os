//! Bindings for Sbi Ecalls to hypervisor based on https://www.scs.stanford.edu/~zyedidia/docs/riscv/riscv-sbi.pdf

const SbiError = error{ FAILED, NOT_SUPPORTED, INVALID_PARAM, DENIED, INVALID_ADDRESS, ALREADY_AVAILABLE, ALREADY_STARTED, ALREADY_STOPPED };

/// Generic ecall binding with args passed as a slice
fn ecall(eid: i32, fid: i32, args: []const u64) i64!SbiError {
    var a0: u64 = if (args.len > 0) args[0] else 0;
    var a1: u64 = if (args.len > 1) args[1] else 0;
    const a2: u64 = if (args.len > 2) args[2] else 0;
    const a3: u64 = if (args.len > 3) args[3] else 0;
    const a4: u64 = if (args.len > 4) args[4] else 0;
    const a5: u64 = if (args.len > 5) args[5] else 0;
    const a6: u64 = @bitCast(@as(i64, fid));
    const a7: u64 = @bitCast(@as(i64, eid));

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

    const error_code: i64 = @bitCast(a0);
    const value: i64 = @bitCast(a1);

    return switch (error_code) {
        0 => value,
        -1 => SbiError.FAILED,
        -2 => SbiError.NOT_SUPPORTED,
        -3 => SbiError.INVALID_PARAM,
        -4 => SbiError.DENIED,
        -5 => SbiError.INVALID_ADDRESS,
        -6 => SbiError.ALREADY_AVAILABLE,
        -7 => SbiError.ALREADY_STARTED,
        -8 => SbiError.ALREADY_STOPPED,
        _ => unreachable
    };
}
