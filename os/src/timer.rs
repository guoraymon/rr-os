use crate::{riscv::get_time, sbi::time::set_timer};

const TICKS_PER_SEC: usize = 100;

/// The frequency of timer interrupt.
pub const CLOCK_FREQ: usize = 12500000;

/// Set next trigger.
pub fn set_next_trigger() {
    set_timer(get_time() + (CLOCK_FREQ / TICKS_PER_SEC) as u64);
}
