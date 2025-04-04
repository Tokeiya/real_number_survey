use super::mantissa::Mantissa;
use crate::build_flot_error::BuildFlotError;

pub const NORM_FLG: u32 = 0x01_00_00_00;

pub const BIT_MASK: [u32; 24] = [
	0x00_00_00_01,
	0x00_00_00_02,
	0x00_00_00_04,
	0x00_00_00_08,
	0x00_00_00_10,
	0x00_00_00_20,
	0x00_00_00_40,
	0x00_00_00_80,
	0x00_00_01_00,
	0x00_00_02_00,
	0x00_00_04_00,
	0x00_00_08_00,
	0x00_00_10_00,
	0x00_00_20_00,
	0x00_00_40_00,
	0x00_00_80_00,
	0x00_01_00_00,
	0x00_02_00_00,
	0x00_04_00_00,
	0x00_08_00_00,
	0x00_10_00_00,
	0x00_20_00_00,
	0x00_40_00_00,
	0x00_80_00_00, //If this bit was raised then the mantissa was normal, otherwise subnormal.
];

pub struct F32Mantissa(u32);

impl TryFrom<u32> for F32Mantissa {
	type Error = BuildFlotError<u32>;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl<'a> From<&'a [bool; 24]> for F32Mantissa {
	fn from(value: &'a [bool; 24]) -> Self {
		todo!()
	}
}

impl std::ops::Index<usize> for F32Mantissa {
	type Output = bool;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl Mantissa<24> for F32Mantissa {
	type Underlying = u32;

	fn effective_precision(&self) -> usize {
		todo!()
	}

	fn underlying(&self) -> Self::Underlying {
		todo!()
	}

	fn to_array(&self) -> [bool; 24] {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::ops::Index;
	use std::panic::AssertUnwindSafe;

	fn indexing_assert<T: Index<usize, Output = bool>>(actual: &T, base: bool, inverted: usize) {
		for i in 0..24 {
			if (i == inverted) {
				assert_eq!(actual[i], !base);
			} else {
				assert_eq!(actual[i], base);
			}
		}

		let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
			let _ = actual[24];
		}));

		assert!(result.is_err());
	}
	#[test]
	pub fn try_from_u32() {}
}
