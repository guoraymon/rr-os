use crate::sbi::{
    dbcn::debug_console_write_byte,
    srst::{system_reset, ResetReason, ResetType},
};

pub fn console_write_byte(byte: u8) {
    debug_console_write_byte(byte);
}

pub fn shutdown(failure: bool) -> ! {
    if !failure {
        system_reset(ResetType::Shutdown, ResetReason::NoReason);
    } else {
        system_reset(ResetType::Shutdown, ResetReason::SystemFailure);
    }
    unreachable!()
}
