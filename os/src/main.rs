#![no_std]
#![no_main]
#![feature(naked_functions)]

core::arch::global_asm!(include_str!("entry.S"));

mod console;
mod sbi;
mod sys;
mod trap;

#[link_section = ".data"]
#[no_mangle]
pub static APP_0: [u8; include_bytes!(
    "../../app/hello_world/target/riscv64gc-unknown-none-elf/release/hello_world.bin"
)
.len()] = *include_bytes!(
    "../../app/hello_world/target/riscv64gc-unknown-none-elf/release/hello_world.bin"
);

#[no_mangle]
fn rust_main() {
    trap::init();

    let app_0_start = APP_0.as_ptr() as usize;
    println!("app_0_start: {:#x}", app_0_start);

    unsafe {
        // 写入 sepc 寄存器（跳转地址）
        core::arch::asm!("csrw sepc, {}", in(reg) app_0_start);

        // 修改 sstatus：清除 SPP 位（使 sret 切换到用户模式）
        let mut sstatus: usize;
        core::arch::asm!("csrr {}, sstatus", out(reg) sstatus);
        sstatus &= !(1 << 8); // SPP = 0
        core::arch::asm!("csrw sstatus, {}", in(reg) sstatus);

        // 执行 sret，跳转到 user_program
        core::arch::asm!("sret", options(noreturn));
    }
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
