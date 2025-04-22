use core::arch::asm;

use crate::{print, println, sys};

pub fn init() {
    let stvec_val = trap_handler as usize;
    unsafe {
        asm!("csrw stvec, {}", in(reg) stvec_val);
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn trap_handler() {
    unsafe {
        core::arch::naked_asm!(
            "addi sp, sp, -2*8",
            // 保存 sepc 寄存器
            "csrr t0, sepc",
            "sd t0, 0(sp)",
            // 保存 ra 寄存器
            "sd x1, 1*8(sp)",

            "call {entry}",

            // 恢复 sepc 寄存器
            "ld t0, 0(sp)",
            "addi t0, t0, 4", // 跳转到 ecall 之后的指令
            "csrw sepc, t0",
            // 恢复 ra 寄存器
            "ld x1, 1*8(sp)",
            "addi sp, sp, 2*8",

            "sret",
            entry = sym trap_entry
        );
    }
}

#[no_mangle]
fn trap_entry() {
    let syscall_id: usize;
    let arg0: usize;
    let arg1: usize;
    let arg2: usize;
    let ra: usize;

    unsafe {
        // 先保存参数
        asm!("mv {0}, a0", out(reg) arg0);
        asm!("mv {0}, a1", out(reg) arg1);
        asm!("mv {0}, a2", out(reg) arg2);
        // 然后保存 syscall_id
        asm!("mv {0}, a7", out(reg) syscall_id);
        asm!("mv {0}, ra", out(reg) ra);
    }
    // println!(
    //     "[kernel] syscall_id: {:#x}, arg0: {:#x}, arg1: {:#x}, arg2: {:#x}, ra: {:#x}",
    //     syscall_id, arg0, arg1, arg2, ra
    // );

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
    // println!(
    //     "[kernel] TrapHandler called, scause: {:#x}, stval: {:#x}",
    //     scause, stval
    // );

    match scause {
        // Environment call from U-mode
        0x8 => match syscall_id {
            64 => match arg0 {
                1 => {
                    let buffer = unsafe { core::slice::from_raw_parts(arg1 as *const u8, arg2) };
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
        },
        _ => panic!("Unsupported scause: {:#x}, stval: {:#x}", scause, stval),
    }
}
