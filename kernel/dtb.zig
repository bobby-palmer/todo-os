//! RISC-V (D)evice (T)ree (B)lob parser

const std = @import("std");

const Header = struct {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,

    /// Parse dtb header given pointer to start of the blob
    fn parse(dtb: usize) Header {
        const buffer: [*]const u32 = @ptrFromInt(dtb);

        const header = Header {
            .magic = std.mem.bigToNative(u32, buffer[0]),
            .totalsize = std.mem.bigToNative(u32, buffer[1]),
            .off_dt_struct = std.mem.bigToNative(u32, buffer[2]),
            .off_dt_strings = std.mem.bigToNative(u32, buffer[3]),
            .off_mem_rsvmap = std.mem.bigToNative(u32, buffer[4]),
            .version = std.mem.bigToNative(u32, buffer[5]),
            .last_comp_version = std.mem.bigToNative(u32, buffer[6]),
            .boot_cpuid_phys = std.mem.bigToNative(u32, buffer[7]),
            .size_dt_strings = std.mem.bigToNative(u32, buffer[8]),
            .size_dt_struct = std.mem.bigToNative(u32, buffer[9]),
        };

        return header;
    }
};
