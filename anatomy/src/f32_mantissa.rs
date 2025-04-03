use super::bit_array::BitArray;
use super::mantissa::Mantissa;
use std::ops::Index;

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
	type Error = ();

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
		[(); 23]:,
	{
		todo!()
	}

	fn is_normal(&self) -> bool {
		todo!()
	}
}
