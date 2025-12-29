const std = @import("std");

const DebugConsole = @import("DebugConsole.zig");

/// Boot hart entry point
export fn kmain(_: u64, _: [*]const u64) noreturn {
    @panic("BOOT");

    // while (true) {} // TODO
}

pub fn panic(message: []const u8, _: ?*std.builtin.StackTrace, _: ?usize) noreturn {
    var writer = DebugConsole.writer();

    _ = writer.print("=== Kernel Panic! ===\n", .{}) catch {};
    _ = writer.print("Error: {s}\n", .{message}) catch {};

    while (true) {}
}
