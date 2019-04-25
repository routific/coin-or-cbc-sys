extern crate coin_or_cbc_sys;

use std::ffi::CStr;

#[test]
fn print_version() {
    let version = unsafe { CStr::from_ptr(coin_or_cbc_sys::Cbc_getVersion()).to_string_lossy() };
    println!("version: {}", version);
}
