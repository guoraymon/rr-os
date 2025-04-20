use crate::sbi::eid_from_str;

use super::sbi_call_1;

pub fn debug_console_write_byte(byte: u8) {
    sbi_call_1(eid_from_str("DBCN"), 0x2, byte as u32);
}
