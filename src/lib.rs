#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;
extern crate header_slice;

mod base;
mod macros;
pub mod string;
#[cfg(test)]
pub mod test;
pub mod vec;
