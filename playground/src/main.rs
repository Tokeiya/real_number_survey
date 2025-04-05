#![feature(unboxed_closures, fn_traits)]

mod f32_anatomy;

use std::env::Args;
use std::ptr::dangling;

fn main() {
	println!("{:x}", 1u32 << 24)
}
