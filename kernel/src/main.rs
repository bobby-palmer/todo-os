#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[inline]
pub fn sbi_putchar(c: u8) {
    // a0 = character (c)
    // a6 = function ID (1 for PutChar)
    // a7 = extension ID (1 for Console Extension)
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") c as u64,
            in("a6") 0x1u64,  // Function ID
            in("a7") 0x1u64,  // Extension ID
            options(nostack)
        );
    }
}

#[unsafe(no_mangle)]
extern "C" fn _kmain() {
    sbi_putchar(b'h');
    sbi_putchar(b'e');
    sbi_putchar(b'l');
    sbi_putchar(b'l');
    sbi_putchar(b'o');
    loop{}
}
