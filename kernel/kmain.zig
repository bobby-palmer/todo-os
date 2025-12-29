const std = @import("std");

const DebugConsole = @import("DebugConsole.zig");

/// Boot hart entry point
export fn kmain(_: usize, _: usize) noreturn {
    while (true) {}
}

/// Global kernel panic handler. Add more info as needed
pub fn panic(message: []const u8, _: ?*std.builtin.StackTrace, _: ?usize) noreturn {
    var writer = DebugConsole.writer();

    _ = writer.print("=== Kernel Panic! ===\n", .{}) catch {};
    _ = writer.print("Error: {s}\n", .{message}) catch {};

    while (true) {}
}
