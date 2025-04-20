use crate::sbi::eid_from_str;

use super::sbi_call_2;

pub enum ResetType {
    Shutdown = 0x0,
}

pub enum ResetReason {
    NoReason = 0x0,
    SystemFailure = 0x1,
}

pub fn system_reset(reset_type: ResetType, reset_reason: ResetReason) {
    sbi_call_2(
        eid_from_str("SRST"),
        0x0,
        reset_type as u32,
        reset_reason as u32,
    );
}
