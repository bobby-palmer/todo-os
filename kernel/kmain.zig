const sbi = @import("sbi.zig");

/// Boot hart entry point
export fn kmain(_: u64, _: [*] const u64) noreturn {
    sbi.Legacy.console_putchar('H') catch unreachable;
    while (true) {} // TODO
}
