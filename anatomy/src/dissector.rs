use super::mantissa::Mantissa;
use std::fmt::Debug;

pub trait Dissector<const N: usize, T: Copy> {
	type Value: Copy + Debug;

	type Exponent: Copy + Debug;

	type Mantissa: Mantissa<N, T> + Debug
	where
		[(); N]: Sized,
		[(); N - 1]: Sized;

	fn value(&self) -> Self::Value;
	fn is_nan(&self) -> bool;
	fn is_infinite(&self) -> bool;
	fn is_finite(&self) -> bool;
	fn is_zero(&self) -> bool;
	fn is_subnormal(&self) -> bool;
	fn is_normal(&self) -> bool;
	fn is_sign_negative(&self) -> bool;
	fn is_sign_positive(&self) -> bool;
	fn exponent(&self) -> Self::Exponent
	where
		[(); N - 1]:;
	fn mantissa(&self) -> Self::Mantissa
	where
		[(); N - 1]:;

	// This part is fluid and not tested
	// value:<round trip>
	// sign:<+/->　zero:bool nan:⛔|✅ inf:bool finite:bool norm:bool sub_norm:bool
	// mantissa:<binary format and hex>
	// exponent:<integer with sign>
	fn describe(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		todo!()
	}
}
