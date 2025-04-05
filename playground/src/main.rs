#![cfg_attr(test, feature(dec2flt))]

mod f32_anatomy;

use std::env::Args;
use std::ptr::dangling;

fn main() {
	dbg!(f32_anatomy::extract_mantissa(f32::MIN_POSITIVE / 2.0));
}
