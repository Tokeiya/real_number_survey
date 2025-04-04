use std::fmt::Debug;
use std::ops::Index;

pub trait Mantissa<const N: usize>
where
	Self: Index<usize, Output = bool>,
{
	type Underlying: Debug + Copy;
	fn effective_precision(&self) -> usize;
	fn underlying(&self) -> Self::Underlying;
	fn to_array(&self) -> [bool; N];
}
