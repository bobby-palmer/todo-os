const c = @import("driver/ns16550.zig");

/// Boot hart entry point
export fn kmain(_: u64, _: [*] const u64) noreturn {
    c.init();
    c.putc('H');
    while (true) {} // TODO
}
