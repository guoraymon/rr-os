#![no_std]
#![no_main]

mod console;
mod sbi;
mod sys;

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
fn rust_main() {
    println!("Hello, world!");
    sys::shutdown(false);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
