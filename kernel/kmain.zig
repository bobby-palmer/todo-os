const std = @import("std");
const dtb = @import("dtb");

const DebugConsole = @import("DebugConsole.zig");


/// Boot hart entry point
export fn kmain(_: usize, device_tree_blob: usize) noreturn {
    const blob_size = 
        dtb.totalSize(@ptrFromInt(device_tree_blob)) catch unreachable;

    var writer = DebugConsole.writer();

    writer.print("totalSize: {d}\n", .{blob_size}) catch unreachable;

    while (true) {}
}

/// Global kernel panic handler. Add more info as needed
pub fn panic(message: []const u8, _: ?*std.builtin.StackTrace, _: ?usize) noreturn {
    var writer = DebugConsole.writer();

    _ = writer.print("=== Kernel Panic! ===\n", .{}) catch {};
    _ = writer.print("Error: {s}\n", .{message}) catch {};

    while (true) {}
}
