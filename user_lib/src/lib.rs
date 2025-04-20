#![no_std]
#![feature(linkage)]

pub mod console;
mod syscall;

use syscall::sys_exit;

#[no_mangle]
fn _start() -> ! {
    sys_exit(main());
    unreachable!();
}

#[no_mangle]
#[linkage = "weak"]
pub fn main() -> i32 {
    0
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
    syscall::sys_exit(1);
    unreachable!();
}
