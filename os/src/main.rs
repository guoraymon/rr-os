#![no_std]
#![no_main]
#![feature(naked_functions)]

// mod batch;
mod console;
mod sbi;
mod sys;
mod task;
mod trap;

core::arch::global_asm!(include_str!("boot.S"));

#[no_mangle]
fn rust_main() {
    trap::init();
    task::init();
    task::run();
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
