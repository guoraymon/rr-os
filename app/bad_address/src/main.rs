#![no_std]
#![no_main]

extern crate user_lib;

#[no_mangle]
fn main() {
    unsafe {
        (0x0 as *mut u8).write_volatile(0);
    }
}
