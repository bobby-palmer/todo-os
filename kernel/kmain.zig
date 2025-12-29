const DebugConsole = @import("DebugConsole.zig");

/// Boot hart entry point
export fn kmain(_: u64, _: [*] const u64) noreturn {
    var writer = DebugConsole.writer();

    writer.print("Hello {x}\n", .{0xd00dfeed}) catch unreachable;

    while (true) {} // TODO
}
