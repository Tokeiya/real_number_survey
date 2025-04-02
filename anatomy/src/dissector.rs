use super::mantissa::Mantissa;

pub trait Dissector<const N: usize> {
	type Target: Copy;

	type Exponent: Copy;

	type Mantissa: Mantissa<N>
	where
		[(); N]:;

	fn is_nan(&self) -> bool;
	fn is_infinite(&self) -> bool;
	fn is_finite(&self) -> bool;
	fn is_zero(&self) -> bool;
	fn is_subnormal(&self) -> bool;
	fn is_normal(&self) -> bool;
	fn is_sign_negative(&self) -> bool;
	fn is_sign_positive(&self) -> bool;
	fn exponent(&self) -> Self::Exponent;
	fn mantissa(&self) -> Self::Mantissa;

	fn describe_exponent(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result;

	fn describe_mantissa(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result;

	fn describe_target(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result;

	fn describe(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		todo!()
	}
}
