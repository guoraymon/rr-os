#![no_std]
#![no_main]
#![feature(naked_functions)]

#[macro_use]
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
mod utils;

use mm::{frame_allocator::frame_allocator_test, heap_allocator::heap_test, memory_set::remap_test};
use sys::shutdown;

core::arch::global_asm!(include_str!("boot.S"));

#[no_mangle]
fn rust_main() {
    extern "C" {
        fn __text_start();
        fn __text_end();
        fn __rodata_start();
        fn __rodata_end();
        fn __data_start();
        fn __data_end();
        fn __bss_start();
        fn __bss_end();
    }
    println!("[kernel] Hello, world!");
    println!(
        "[kernel] .text [{:#x}, {:#x})",
        __text_start as usize, __text_end as usize
    );
    println!(
        "[kernel] .rodata [{:#x}, {:#x})",
        __rodata_start as usize, __rodata_end as usize
    );
    println!(
        "[kernel] .data [{:#x}, {:#x})",
        __data_start as usize, __data_end as usize
    );
    println!(
        "[kernel] .bss [{:#x}, {:#x})",
        __bss_start as usize, __bss_end as usize
    );

    // trap::init();
    // trap::enable_timer_interrupt();
    // timer::set_next_trigger();
    // task::init();
    // task::run();
    mm::init();
    heap_test();
    frame_allocator_test();
    remap_test();
    shutdown(false);
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
