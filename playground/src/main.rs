#![feature(unboxed_closures, fn_traits)]

use std::env::Args;
use std::ptr::dangling;

struct Foo;

impl Fn<(u32,)> for Foo {
	extern "rust-call" fn call(&self, args: (u32,)) -> u32 {
		let (x,) = args;
		x + 1
	}
}

fn hoge<F: Fn(u32) -> u32>(v: u32, f: F) -> u32 {
	f(v)
}

fn main() {
	let f = f32::NAN;
	dbg!(f.is_finite());
	dbg!("○　　⛔");
}

fn to_bin(x: u32) -> String {
	let mut s = String::new();

	for i in (0..32).rev() {
		if x & (1 << i) != 0 {
			s.push('1');
		} else {
			s.push('0');
		}

		if i % 4 == 0 && i != 0 {
			s.push('_');
		}
	}
	s
}
