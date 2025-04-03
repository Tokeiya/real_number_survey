use super::bit_array::BitArray;
use super::mantissa::Mantissa;
use crate::float_build_error::FloatBuildError;
use std::ops::Index;

const MASK: [u32; 23] = [
	0x00_00_01, 0x00_00_02, 0x00_00_04, 0x00_00_08, 0x00_00_10, 0x00_00_20, 0x00_00_40, 0x00_00_80,
	0x00_01_00, 0x00_02_00, 0x00_04_00, 0x00_08_00, 0x00_10_00, 0x00_20_00, 0x00_40_00, 0x00_80_00,
	0x01_00_00, 0x02_00_00, 0x04_00_00, 0x08_00_00, 0x10_00_00, 0x20_00_00, 0x40_00_00,
];

pub struct F32Mantissa(u32);

impl Index<usize> for F32Mantissa {
	type Output = bool;

	fn index(&self, index: usize) -> &Self::Output {
		todo!()
	}
}

impl From<BitArray<24>> for F32Mantissa {
	fn from(bits: BitArray<24>) -> Self {
		todo!()
	}
}

impl TryFrom<u32> for F32Mantissa {
	type Error = FloatBuildError<u32>;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		todo!()
	}
}

impl Mantissa<24, u32> for F32Mantissa {
	fn underlying(&self) -> u32 {
		todo!()
	}

	fn to_array(&self) -> BitArray<24>
	where
		[(); 23]: Sized,
	{
		todo!()
	}

	fn is_normal(&self) -> bool {
		todo!()
	}
}
