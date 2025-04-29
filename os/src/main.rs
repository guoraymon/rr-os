#![no_std]
#![no_main]
#![feature(naked_functions)]

// mod batch;
mod console;
mod sbi;
mod sys;
mod task;
mod trap;

const MAX_APP_NUM: usize = 3;

const KERNEL_STACK_SIZE: usize = 4096;

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

const USER_STACK_SIZE: usize = 4096;

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static USER_STACKS: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

#[no_mangle]
pub static APPS: [&[u8]; 3] = [
    include_bytes!(
        "../../app/hello_world/target/riscv64gc-unknown-none-elf/release/hello_world.bin"
    ),
    include_bytes!(
        "../../app/bad_address/target/riscv64gc-unknown-none-elf/release/bad_address.bin"
    ),
    include_bytes!(
        "../../app/bad_instructions/target/riscv64gc-unknown-none-elf/release/bad_instructions.bin"
    ),
];

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
