use super::build_flot_error::BuildFlotError;
use super::mantissa::Mantissa;
use std::ops::Index;
pub struct F32Mantissa(u32);

impl F32Mantissa {
	pub fn is_subnormal(&self) -> bool {
		todo!("not implemented");
	}
}

impl TryFrom<u32> for F32Mantissa {
	type Error = BuildFlotError<u32>;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl TryFrom<&[bool]> for F32Mantissa {
	type Error = BuildFlotError<u32>;

	fn try_from(value: &[bool]) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl Index<usize> for F32Mantissa {
	type Output = bool;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl Mantissa<u32> for F32Mantissa {
	fn precision(&self) -> usize {
		todo!()
	}

	fn underlying(&self) -> u32 {
		todo!()
	}

	fn to_array(&self) -> Vec<bool> {
		todo!()
	}
}
