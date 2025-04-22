#![no_std]
#![no_main]

#[unsafe(no_mangle)]
fn _start() {
    // write
    let str = "Hello, world!\n";
    unsafe {
        core::arch::asm!(
            "ecall",
            in("x10") 1,
            in("x11") str.as_ptr(),
            in("x12") str.len(),
            in("x17") 64
        );
    }
    // exit
    unsafe {
        core::arch::asm!(
            "ecall",
            in("x10") 0,
            in("x17") 93
        );
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
