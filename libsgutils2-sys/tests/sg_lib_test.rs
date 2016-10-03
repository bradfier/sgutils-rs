extern crate libsgutils2_sys;

use std::ffi::CStr;
use libsgutils2_sys::sg_lib;

#[test]
fn test_version() {
    unsafe {
        let slice = CStr::from_ptr(sg_lib::sg_lib_version());
        println!("{}", slice.to_str().unwrap());
    }
}
