use core::arch::asm;

/// 启用监督模式全局中断（设置 sstatus.SIE）
pub fn enable_supervisor_interrupts() {
    let mask = 1 << 1; // STE bit
    unsafe {
        core::arch::asm!(
            "csrs sstatus, {}",
            in(reg) mask
        );
    }
}

/// 启用监督模式定时器中断（设置 sie.STIE）
pub fn enable_supervisor_timer_interrupt() {
    let mask = 1 << 5; // STIE bit
    unsafe {
        core::arch::asm!(
            "csrs sie, {}",
            in(reg) mask
        );
    }
}

/// 获取当前系统时间（基于 rdtime 指令）
pub fn get_time() -> u64 {
    let mut time: u64;
    unsafe {
        // 使用 rdtime 指令读取当前系统时间（单位：tick）
        asm!("rdtime {}", out(reg) time);
    }
    time
}
