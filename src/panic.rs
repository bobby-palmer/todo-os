use crate::println;

/// Set the global panic handler for kernel
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!();
    println!("!!! Kernel Panic !!!");
    println!("{:?}", info);
    loop {}
}
