use core::arch::global_asm;

use crate::{
    println, sys,
    trap::{TrapContext, __restore},
    APPS, KERNEL_STACK, USER_STACKS,
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
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub context: TaskContext,
}

pub struct TaskManager {
    num_app: usize,
    tasks: [TaskControlBlock; 16],
    current_task: usize,
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

    fn exit(&mut self) {
        self.tasks[self.current_task].status = TaskStatus::Exited;
    }

    fn find_next_id(&self) -> Option<usize> {
        for i in 0..self.num_app {
            if self.tasks[i].status == TaskStatus::Ready && i != self.current_task {
                return Some(i);
            }
        }
        None
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
}

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(
        current_task_context_ptr: *mut TaskContext,
        next_task_context_ptr: *const TaskContext,
    );
}

pub static mut TASK_MANAGER: TaskManager = {
    TaskManager {
        num_app: APPS.len(),
        tasks: [TaskControlBlock {
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

pub fn init() {
    for (i, app) in APPS.iter().enumerate() {
        let start = 0x80400000 + i * 0x2000;
        println!(
            "app_{}: {:#x}, len: {:#x}, start: {:#x}, k_s: {:#x}, u_s: {:#x}",
            i,
            app.as_ptr() as usize,
            app.len(),
            start,
            KERNEL_STACK[i].data.as_ptr() as usize,
            USER_STACKS[i].data.as_ptr() as usize,
        );
        unsafe {
            core::ptr::copy_nonoverlapping(APPS[i].as_ptr(), start as *mut u8, APPS[i].len());

            let task = &mut TASK_MANAGER.tasks[i];
            task.status = TaskStatus::Ready;
            task.context.ra = __restore as usize;
            task.context.sp = (|| {
                let ptr = (KERNEL_STACK[i].data.as_ptr() as usize
                    - core::mem::size_of::<TrapContext>())
                    as *mut TrapContext;
                *ptr = TrapContext {
                    sstatus: (|| {
                        let mut sstatus: usize;
                        core::arch::asm!("csrr {}, sstatus", out(reg) sstatus);
                        sstatus &= !(1 << 8); // SPP = 0
                        sstatus
                    })(),
                    sepc: start as usize,
                    ra: 0,
                    sp: USER_STACKS[i].data.as_ptr() as usize,
                    a: [0; 8],
                };
                ptr as usize
            })();
        }
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
