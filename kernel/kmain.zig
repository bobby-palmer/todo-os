const std = @import("std");

const DebugConsole = @import("DebugConsole.zig");
const DeviceTree = @import("DeviceTree.zig");

/// Boot hart entry point
export fn kmain(_: usize, dtb: usize) noreturn {
    var writer = DebugConsole.writer();

    const device_tree = DeviceTree.parse(dtb) catch @panic("Device tree");

    var reservations = device_tree.memory_reservations.iterator();

    while (reservations.next()) |reservation| {
        writer.print("start: {x}, size: {}\n", 
            .{reservation.address, reservation.size}) catch unreachable;
    }

    writer.print("Done\n", .{}) catch unreachable;

    while (true) {}
}

/// Global kernel panic handler. Add more info as needed
pub fn panic(message: []const u8, _: ?*std.builtin.StackTrace, _: ?usize) noreturn {
    var writer = DebugConsole.writer();

    _ = writer.print("=== Kernel Panic! ===\n", .{}) catch {};
    _ = writer.print("Error: {s}\n", .{message}) catch {};

    while (true) {}
}
