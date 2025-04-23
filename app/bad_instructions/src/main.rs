#![no_std]
#![no_main]

extern crate user_lib;

#[no_mangle]
fn main() {
    unsafe {
        core::arch::asm!("sret");
    }
    panic!("FAIL: T.T\n");
}
