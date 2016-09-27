#![allow(dead_code)]
#![allow(non_camel_case_types)]

#![feature(libc)]

extern crate libc;

mod sg_lib;
mod sg_cmds;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
