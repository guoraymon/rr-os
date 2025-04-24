use crate::{println, sys::shutdown};

static mut APP_ID: usize = 0;

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

pub fn run() {
    unsafe {
        println!(
            "app_{}: {:#x}, len: {:#x}",
            APP_ID,
            APPS[APP_ID].as_ptr() as usize,
            APPS[APP_ID].len()
        );
        let start = 0x80400000 + APP_ID * 0x2000;
        println!("start: {:#x}", start);
        core::ptr::copy_nonoverlapping(
            APPS[APP_ID].as_ptr(),
            start as *mut u8,
            APPS[APP_ID].len(),
        );
        core::arch::asm!("fence.i"); // 刷新指令缓存

        // 写入 sepc 寄存器（跳转地址）
        core::arch::asm!("csrw sepc, {}", in(reg) start as usize);

        // 修改 sstatus：清除 SPP 位（使 sret 切换到用户模式）
        let mut sstatus: usize;
        core::arch::asm!("csrr {}, sstatus", out(reg) sstatus);
        sstatus &= !(1 << 8); // SPP = 0
        core::arch::asm!("csrw sstatus, {}", in(reg) sstatus);

        // 执行 sret，跳转到 user_program
        core::arch::asm!("sret", options(noreturn));
    }
}

pub fn next() {
    unsafe {
        if APP_ID < APPS.len() - 1 {
            APP_ID += 1;
            run();
        } else {
            shutdown(false);
        }
    }
}
