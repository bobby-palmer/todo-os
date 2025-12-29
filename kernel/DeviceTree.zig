//! RISC-V Device Tree Blob parser

const std = @import("std");
const DeviceTree = @This();

memory_reservations: MemoryReservations,

pub const Error = error {
    BadMagic,
    VersionTooOld,
    VersionTooNew,
};

const spec_version = 17;

pub fn parse(dtb: usize) Error!DeviceTree {
    const header = Header.parse(dtb);

    if (header.magic != 0xd00dfeed) {
        return Error.BadMagic;
    }

    if (header.last_comp_version > spec_version) {
        return Error.VersionTooNew; 
    }

    if (header.version < spec_version) {
        return Error.VersionTooOld;
    }

    const memory_reservations = MemoryReservations.init(
        dtb + header.off_mem_rsvmap
    );

    return DeviceTree {
        .memory_reservations = memory_reservations,
    };
}

const Header = extern struct {
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

    fn parse(dtb: usize) Header {
        var header = @as(*Header, @ptrFromInt(dtb)).*;

        inline for (std.meta.fields(Header)) |field| {
            @field(header, field.name) = std.mem.bigToNative(
                u32, @field(header, field.name)
            );
        }

        return header;
    }
};

const MemoryReservations = struct {
    mem_rsvmap: [*] const u64,

    fn init(mem_rsvmap: usize) MemoryReservations {
        return .{
            .mem_rsvmap = @ptrFromInt(mem_rsvmap)
        };
    }

    pub fn iterator(self: *const MemoryReservations) Iterator {
        return .{ .head = self.mem_rsvmap };
    }

    const Entry = struct {
        address: u64,
        size: u64,
    };

    const Iterator = struct {

        head: [*] const u64,

        pub fn next(self: *Iterator) ?Entry {
            const address = std.mem.bigToNative(u64, self.head[0]);
            const size = std.mem.bigToNative(u64, self.head[1]);

            if (address == 0 and size == 0) {
                return null;
            } else {
                self.head += 2;
                return .{ .address = address, .size = size };
            }
        }
    };
};
