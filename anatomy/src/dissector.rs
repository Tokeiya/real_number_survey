use super::mantissa::Mantissa;

pub trait Dissector<const N: usize> {
	type Value: Copy;

	type Exponent: Copy;

	type Mantissa: Mantissa<N>
	where
		[(); N]:;

	fn value(&self) -> Self::Value;
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

	fn describe_exponent(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;

	fn describe_mantissa(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;

	fn describe_target(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;


	// value:<round trip>
	// sign:<+/->
	// mantissa:<binary format and hex>
	// exponent:<integer with sign>
	// is_nan:bool
	// is_inf:bool
	// is_nomal:bool

	fn describe(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f,"value:")?;
		Self::describe_target(self, f)?;
		writeln!(f,"")?;
		
		todo!()
	}
}
