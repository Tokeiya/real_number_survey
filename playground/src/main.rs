#![cfg_attr(test, feature(dec2flt))]

mod f32_anatomy;

use std::env::Args;
use std::ptr::dangling;

fn main() {
	assert_eq!(crate::f32_anatomy::extract_mantissa(1.0), 0x00_80_00_00);
}
