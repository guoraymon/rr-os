use core::arch::global_asm;

use crate::{
    println, sys,
    trap::{TrapContext, __restore},
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

#[derive(Copy, Clone)]
pub struct Task {
    pub status: TaskStatus,
    pub context: TaskContext,
}

pub struct TaskManager {
    num_app: usize,
    tasks: [Task; 16],
    current_task: usize,
}

const APP_NUM: usize = 3;

pub static mut TASK_MANAGER: TaskManager = {
    TaskManager {
        num_app: APP_NUM,
        tasks: [Task {
            status: TaskStatus::UnInit,
            context: TaskContext {
                ra: 0,
                sp: 0,
                s: [0; 12],
            },
        }; 16],
        current_task: 0,
    }
};

const KERNEL_STACK_SIZE: usize = 0x10000;

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    _data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn top(&self) -> usize {
        self as *const _ as usize + KERNEL_STACK_SIZE
    }
}

#[link_section = ".kernel_stack"]
static KERNEL_STACKS: [KernelStack; APP_NUM] = [KernelStack {
    _data: [0; KERNEL_STACK_SIZE],
}; APP_NUM];

const USER_STACK_SIZE: usize = 0x10000;

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    _data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn top(&self) -> usize {
        self as *const _ as usize + USER_STACK_SIZE
    }
}

#[link_section = ".user_stack"]
static USER_STACKS: [UserStack; APP_NUM] = [UserStack {
    _data: [0; USER_STACK_SIZE],
}; APP_NUM];

pub fn init() {
    const APPS: [&[u8]; APP_NUM] = [
        // include_bytes!(
        //     "../../app/hello_world/target/riscv64gc-unknown-none-elf/release/hello_world.bin"
        // ),
        // include_bytes!(
        //     "../../app/bad_address/target/riscv64gc-unknown-none-elf/release/bad_address.bin"
        // ),
        // include_bytes!(
        //     "../../app/bad_instructions/target/riscv64gc-unknown-none-elf/release/bad_instructions.bin"
        // ),
        include_bytes!("../../app/yield_a/target/riscv64gc-unknown-none-elf/release/yield_a.bin"),
        include_bytes!("../../app/yield_b/target/riscv64gc-unknown-none-elf/release/yield_b.bin"),
        include_bytes!("../../app/yield_c/target/riscv64gc-unknown-none-elf/release/yield_c.bin"),
    ];

    for (i, app) in APPS.iter().enumerate() {
        println!(
            "app_{}: {:#x}, len: {:#x}",
            i,
            app.as_ptr() as usize,
            app.len(),
        );

        let start = 0x80400000 + i * 0x20000;
        let kernel_stack_ptr = KERNEL_STACKS[i].top();
        let user_stack_ptr = USER_STACKS[i].top();
        println!(
            "start: {:#x}, kernel_stack: {:#x}, user_stack: {:#x}",
            start, kernel_stack_ptr, user_stack_ptr
        );

        unsafe {
            core::ptr::copy_nonoverlapping(APPS[i].as_ptr(), start as *mut u8, APPS[i].len());

            let task = &mut TASK_MANAGER.tasks[i];
            task.status = TaskStatus::Ready;
            task.context.ra = __restore as usize;
            task.context.sp = (|| {
                let ptr =
                    (kernel_stack_ptr - core::mem::size_of::<TrapContext>()) as *mut TrapContext;

                let mut trap_context = TrapContext {
                    x: [0; 32],
                    sstatus: (|| {
                        let mut sstatus: usize;
                        core::arch::asm!("csrr {}, sstatus", out(reg) sstatus);
                        sstatus &= !(1 << 8); // SPP = 0
                        sstatus
                    })(),
                    sepc: start as usize,
                };
                trap_context.x[2] = user_stack_ptr;

                *ptr = trap_context;
                ptr as usize
            })();
        }
    }
}

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(
        current_task_context_ptr: *mut TaskContext,
        next_task_context_ptr: *const TaskContext,
    );
}

impl TaskManager {
    pub fn run(&mut self) {
        let mut init_task_context = TaskContext {
            ra: 0,
            sp: 0,
            s: [0; 12],
        };
        let next_task_context_ptr =
            &mut self.tasks[self.current_task].context as *const TaskContext;

        self.tasks[self.current_task].status = TaskStatus::Running;

        println!("run: {}", self.current_task);
        unsafe {
            __switch(
                &mut init_task_context as *mut TaskContext,
                next_task_context_ptr,
            );
        }
    }

    fn suspend(&mut self) {
        self.tasks[self.current_task].status = TaskStatus::Ready;
    }

    fn find_next_id(&self) -> Option<usize> {
        for i in 0..self.num_app {
            if self.tasks[i].status == TaskStatus::Ready && i != self.current_task {
                return Some(i);
            }
        }
        // if no ready task, try run the current task again
        if self.tasks[self.current_task].status == TaskStatus::Ready {
            return Some(self.current_task);
        } else {
            return None;
        }
    }

    fn next(&mut self) {
        if let Some(i) = self.find_next_id() {
            let current_task_cx_ptr =
                &mut self.tasks[self.current_task].context as *mut TaskContext;

            let next_task_context_ptr = &self.tasks[i].context as *const TaskContext;

            self.current_task = i;
            self.tasks[self.current_task].status = TaskStatus::Running;

            println!("switch: {}", self.current_task);
            unsafe {
                __switch(current_task_cx_ptr, next_task_context_ptr);
            }
        } else {
            println!("no task to run");
            sys::shutdown(false);
        }
    }

    fn exit(&mut self) {
        self.tasks[self.current_task].status = TaskStatus::Exited;
    }
}

pub fn run() {
    unsafe {
        TASK_MANAGER.run();
    }
}

pub fn suspend_current_and_run_next() {
    unsafe {
        TASK_MANAGER.suspend();
        TASK_MANAGER.next();
    }
}

pub fn exit_current_and_run_next() {
    unsafe {
        TASK_MANAGER.exit();
        TASK_MANAGER.next();
    }
}
