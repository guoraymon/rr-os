use crate::sbi::eid_from_str;

use super::sbi_call_1_u64;

pub fn set_timer(stime_value: u64) {
    sbi_call_1_u64(eid_from_str("TIME"), 0x0, stime_value);
}
