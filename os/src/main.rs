#![no_std]
#![no_main]
#![feature(naked_functions)]

use mm::heap_test;

extern crate alloc;

// mod batch;
mod console;
mod mm;
mod riscv;
mod sbi;
mod sys;
mod task;
mod timer;
mod trap;

core::arch::global_asm!(include_str!("boot.S"));

#[no_mangle]
fn rust_main() {
    // trap::init();
    // trap::enable_timer_interrupt();
    // timer::set_next_trigger();
    // task::init();
    // task::run();
    mm::init();
    heap_test();
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
