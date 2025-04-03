use super::bit_array::BitArray;
use std::ops::Index;
pub trait Mantissa<const N: usize, T: Copy>:
	Index<usize, Output = bool> + From<BitArray<N>> + TryFrom<T>
where
	[(); N - 1]:,
{
	fn underlying(&self) -> T;
	fn to_array(&self) -> BitArray<N>
	where
		[(); N - 1]:;

	fn is_normal(&self) -> bool;
	fn is_subnormal(&self) -> bool {
		!self.is_normal()
	}
}
