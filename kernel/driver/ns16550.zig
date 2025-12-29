//! Driver for QEMU default virt-uart

const std = @import("std");

const Register = enum(u8) {
    RBR_THR = 0x00,  // Receive buffer (read)
    IER = 0x01,  // Interrupt enable
    FCR = 0x02,  // FIFO control (write)
    LCR = 0x03,  // Line control
    LSR = 0x05,  // Line status
};

const LSR = packed struct {
    dr: bool,      // Data ready
    oe: bool,      // Overrun error
    pe: bool,      // Parity error
    fe: bool,      // Framing error
    bi: bool,      // Break interrupt
    thre: bool,    // THR empty
    temt: bool,    // Transmitter empty
    err_fifo: bool,
};

// hard coding this for now :sob
const base_addr: usize = 0x10000000;

// W/R helpers, abstract later

fn reg_write(reg: Register, val: u8) void {
    const ptr: *volatile u8 = @ptrFromInt(base_addr + @intFromEnum(reg));
    ptr.* = val;
}

fn reg_read(reg: Register) u8 {
    const ptr: *volatile u8 = @ptrFromInt(base_addr + @intFromEnum(reg));
    return ptr.*;
}

fn read_lsr() LSR {
    return @bitCast(reg_read(.LSR));
}

pub fn init() void {
    reg_write(.IER, 0x00);           // Disable interrupts
    reg_write(.LCR, 0x80);           // Enable DLAB
    reg_write(.RBR_THR, 0x01);           // Divisor low (115200)
    reg_write(.IER, 0x00);           // Divisor high
    reg_write(.LCR, 0x03);           // 8N1, disable DLAB
    reg_write(.FCR, 0x07);           // Enable and clear FIFOs
}

pub fn putc(c: u8) void {
    while (!read_lsr().thre) {}
    reg_write(.RBR_THR, c);
}

pub fn getc() u8 {
    while (!read_lsr().dr) {}
    return reg_read(.RBR_THR);
}
