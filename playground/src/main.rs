#![feature(unboxed_closures, fn_traits)]

use std::env::Args;
use std::ptr::dangling;

fn main() {
	println!("{:x}", 1u32 << 24)
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
