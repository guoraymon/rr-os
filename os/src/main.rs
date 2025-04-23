#![no_std]
#![no_main]
#![feature(naked_functions)]

use sys::shutdown;

mod batch;
mod console;
mod sbi;
mod sys;
mod trap;

const BOOT_STACK_SIZE: usize = 4096;

#[repr(align(4096))]
struct BootStack {
    _data: [u8; BOOT_STACK_SIZE],
}

static BOOT_STACK: BootStack = BootStack {
    _data: [0; BOOT_STACK_SIZE],
};

#[no_mangle]
#[link_section = ".text.entry"]
fn _start() {
    unsafe {
        core::arch::asm!(
            "la sp, {boot_stack}",
            "call rust_main",
            boot_stack = sym BOOT_STACK
        );
    }
}

#[no_mangle]
fn rust_main() {
    trap::init();
    batch::run();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("Panicked: {}", info.message());
    }
    sys::shutdown(true);
}
