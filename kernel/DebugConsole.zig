//! Early debug printing wrapper around open SBI

const std = @import("std");
const sbi = @import("sbi.zig");

const Writer = std.io.Writer;
const Error = Writer.Error;

pub fn writer() Writer {
    return Writer{
        .vtable = &Writer.VTable{
            .drain = drain,
        },
        // Un-buffered writer
        .buffer = &.{},
    };
}

fn drain(_: *Writer, data: []const []const u8, splat: usize) Error!usize {
    var written: usize = 0;

    for (0..data.len - 1) |idx| {
        for (data[idx]) |ch| {
            _ = sbi.Legacy.console_putchar(ch) catch {
                return Error.WriteFailed;
            };

            written += 1;
        }
    }

    for (0..splat) |_| {
        for (data[data.len - 1]) |ch| {
            _ = sbi.Legacy.console_putchar(ch) catch {
                return Error.WriteFailed;
            };

            written += 1;
        }
    }

    return written;
}
