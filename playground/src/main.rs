#![feature(unboxed_closures, fn_traits)]

use std::env::Args;
use std::ptr::dangling;

struct Foo;

fn hoge<F: Fn(u32) -> u32>(v: u32, f: F) -> u32 {
	f(v)
}

fn main() {
	let a = 1u32;
	let b = !a;

	dbg!(a);
	dbg!(b);
}
