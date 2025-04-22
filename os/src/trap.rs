use core::arch::asm;

use crate::{print, println, sys};

pub fn init() {
    let stvec_val = (trap_handler as usize & !0x3) | 0x1;
    unsafe {
        asm!("csrw stvec, {}", in(reg) stvec_val);
    }
}

#[no_mangle]
pub extern "C" fn trap_handler() {
    // 一定要在处理异常之前保存寄存器
    let syscall_id: usize;
    let arg0: usize;
    let arg1: usize;
    let arg2: usize;

    unsafe {
        // 先保存参数
        asm!("mv {0}, a0", out(reg) arg0);
        asm!("mv {0}, a1", out(reg) arg1);
        asm!("mv {0}, a2", out(reg) arg2);
        // 然后保存 syscall_id
        asm!("mv {0}, a7", out(reg) syscall_id);
    }
    println!(
        "syscall_id: {:#x}, arg0: {:#x}, arg1: {:#x}, arg2: {:#x}",
        syscall_id, arg0, arg1, arg2
    );

    let mut scause: usize; // 异常原因寄存器 Supervisor Cause Register
    let mut stval: usize; // 异常相关值寄存器 Supervisor Trap Value Register
    unsafe {
        asm!(
            "csrr {0}, scause",
            "csrr {1}, stval",
            out(reg) scause,
            out(reg) stval
        );
    }
    println!(
        "TrapHandler called, scause: {:#x}, stval: {:#x}",
        scause, stval
    );

    match scause {
        // Environment call from U-mode
        0x8 => {
            match syscall_id {
                64 => match arg0 {
                    1 => {
                        let buffer =
                            unsafe { core::slice::from_raw_parts(arg1 as *const u8, arg2) };
                        print!("{}", core::str::from_utf8(buffer).unwrap());
                    }
                    _ => panic!("Unsupported fd: {}", arg0),
                },
                93 => {
                    sys::shutdown(false);
                }
                _ => {
                    panic!("Unsupported syscall_id: {:#x}", syscall_id);
                }
            }

            // 返回用户态前，sepc += 4 跳过 ecall 指令
            let mut sepc: usize;
            unsafe {
                // 获取当前的 sepc 值
                asm!("csrr {}, sepc", out(reg) sepc);

                // 跳过 ecall 指令，增加 4
                sepc += 4;

                // 将新的 sepc 值写回
                asm!("csrw sepc, {}", in(reg) sepc);

                // 执行 sret 返回到用户态
                core::arch::asm!("sret", options(noreturn));
            }
        }
        _ => panic!("Unsupported scause: {:#x}, stval: {:#x}", scause, stval),
    }
}
