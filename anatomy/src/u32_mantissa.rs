use super::mantissa::Mantissa;
use crate::bit_array::BitArray;

pub fn from_f32_norm(mantissa: f32) -> u32 {
	todo!()
}

pub fn from_f32_subnorm(mantissa: f32) -> u32 {
	todo!()
}

impl Mantissa<24> for u32 {
	fn to_array(&self) -> BitArray<24>
	where
		[(); 24 - 1]:,
	{
		todo!()
	}

	fn is_normal(&self) -> bool {
		todo!()
	}
}

#[cfg(test)]
mod test {
	use super::*;
}
