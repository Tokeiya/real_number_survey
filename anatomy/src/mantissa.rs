use super::bit_array::BitArray;
pub trait Mantissa<const N: usize> {
	fn to_array(&self) -> BitArray<N>
	where
		[(); N - 1]:;

	fn is_normal(&self) -> bool;
	fn is_subnormal(&self) -> bool {
		!self.is_normal()
	}
}
