pub mod dbcn;
pub mod srst;

/// Converts SBI EID from str.
fn eid_from_str(name: &str) -> u32 {
    match *name.as_bytes() {
        [a] => u32::from_be_bytes([0, 0, 0, a]),
        [a, b] => u32::from_be_bytes([0, 0, a, b]),
        [a, b, c] => u32::from_be_bytes([0, a, b, c]),
        [a, b, c, d] => u32::from_be_bytes([a, b, c, d]),
        _ => unreachable!(),
    }
}

fn sbi_call_1(eid: u32, fid: u32, arg0: u32) {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") eid,
            in("a6") fid,
            inlateout("a0") arg0 => _,
            lateout("a1") _,
        );
    }
}

fn sbi_call_2(eid: u32, fid: u32, arg0: u32, arg1: u32) {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a7") eid,
            in("a6") fid,
            inlateout("a0") arg0 => _,
            inlateout("a1") arg1 => _,
        );
    }
}
