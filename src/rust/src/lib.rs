#![allow(unused, non_camel_case_types)]

extern crate libc;

pub mod atom;
pub mod avmpack;
pub mod opcodes;
pub mod term;
pub mod mapped_file;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
