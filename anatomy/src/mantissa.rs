use std::fmt::Debug;
use std::ops::Index;

pub trait Mantissa<T: Copy + Debug>: Index<usize, Output = bool> {
	fn precision(&self) -> usize;
	fn underlying(&self) -> T;
	fn to_array(&self) -> Vec<bool>;
}
