use core::arch::{asm, global_asm};

use crate::{print, println, task};

#[repr(C)]
#[derive(Debug)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}

global_asm!(include_str!("trap.S"));

extern "C" {
    pub fn __trap();
}

extern "C" {
    pub fn __restore(trap_context: *const TrapContext);
}

pub fn init() {
    unsafe {
        asm!("csrw stvec, {}", in(reg) __trap as usize);
    }
}

#[no_mangle]
fn trap_handler(trap_context: &mut TrapContext) -> &mut TrapContext {
    let mut scause: usize; // Supervisor Cause Register
    let mut stval: usize; // Supervisor Trap Value Register
    unsafe {
        asm!(
            "csrr {0}, scause",
            "csrr {1}, stval",
            out(reg) scause,
            out(reg) stval
        );
    }
    // println!("trap_handle: scause: {:#x}, stval: {:#x}", scause, stval);

    match scause {
        // Illegal instruction
        0x2 => {
            println!("Illegal instruction: {:#x}", stval);
            task::exit_current_and_run_next();
        }
        // Store/AMO access fault
        0x7 => {
            println!("Store/AMO access fault: {:#x}", stval);
            task::exit_current_and_run_next();
        }
        // Environment call from U-mode
        0x8 => {
            let syscall_id = trap_context.x[17];
            let arg0: usize = trap_context.x[10];
            let arg1 = trap_context.x[11];
            let arg2 = trap_context.x[12];
            // println!(
            //     "syscall_id: {}, arg0: {:#x}, arg1: {:#x}, arg2: {:#x}",
            //     syscall_id, arg0, arg1, arg2
            // );

            trap_context.sepc += 4;
            trap_context.x[10] = match syscall_id {
                64 => match arg0 {
                    1 => {
                        let buffer =
                            unsafe { core::slice::from_raw_parts(arg1 as *const u8, arg2) };
                        print!("{}", core::str::from_utf8(buffer).unwrap());
                        buffer.len()
                    }
                    _ => panic!("Unsupported fd: {}", arg0),
                },
                93 => {
                    println!("Application exit");
                    task::exit_current_and_run_next();
                    0
                }
                124 => {
                    println!("Application yield");
                    task::suspend_current_and_run_next();
                    println!("Application resumed");
                    0
                }
                _ => {
                    panic!("Unsupported syscall_id: {}", syscall_id);
                }
            };
        }
        _ => {
            panic!("Unsupported scause: {:#x}, stval: {:#x}", scause, stval);
        }
    }
    trap_context
}
