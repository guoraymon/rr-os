#![no_std]
#![feature(linkage)]

pub mod console;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
fn _start() -> ! {
    exit(main());
    unreachable!();
}

#[no_mangle]
#[linkage = "weak"]
pub fn main() -> i32 {
    0
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    syscall::sys_write(fd, buffer)
}

pub fn exit(xstate: i32) -> isize {
    syscall::sys_exit(xstate)
}

pub fn yield_() -> isize {
    syscall::sys_yield()
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
    syscall::sys_exit(-1);
    unreachable!();
}
