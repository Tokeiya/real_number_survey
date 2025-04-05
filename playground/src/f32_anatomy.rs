pub const BIT_MASK: [u32; 23] = [
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
];

const ECONOMIZE: u32 = 0x00_80_00_00;
const MANTISSA_MASK: u32 = 0x00_7F_FF_FF;
const EXPONENT_MASK: u32 = 0x7F_80_00_00;
const EXPONENT_SHIFT: u32 = 23;
const SIGN_MASK: u32 = 0x80_00_00_00;
const SIGN_SHIFT: u32 = 31;

pub fn get_sign(value: f32) -> bool {
	let tmp = value.to_bits();
	(tmp & SIGN_MASK) != 0
}

pub fn extract_exponent(value: f32) -> u8 {
	((value.to_bits() & EXPONENT_MASK) >> EXPONENT_SHIFT) as u8
}

pub fn get_exponent(value: f32) -> Option<i8> {
	let exp = extract_exponent(value);
	todo!()
}

pub fn extract_mantissa(value: f32) -> u32 {
	value.to_bits() & MANTISSA_MASK
}
pub fn get_mantissa(value: f32) -> u32 {
	todo!("not implemented");
}

pub fn get_mantissa_array(value: f32) -> [bool; 24] {
	todo!("not implemented");
}

#[cfg(test)]
mod tests {

	use crate::f32_anatomy as target;
	use crate::f32_anatomy::ECONOMIZE;
	use core::num::dec2flt::float::RawFloat;

	#[test]
	fn extract_exponent() {
		assert_eq!(target::extract_exponent(0.0), 0);
		assert_eq!(target::extract_exponent(1.0), 127);
		assert_eq!(target::extract_exponent(0.75), 126);
		assert_eq!(target::extract_exponent(0.5), 132);
	}

	#[test]
	fn extract_mantissa() {
		assert_eq!(target::extract_mantissa(1.0), ECONOMIZE);
		assert_eq!(target::extract_mantissa(0.5), ECONOMIZE);
		assert_eq!(target::extract_mantissa(0.75), 0x0040_0000);
	}

	#[test]
	fn get_sign() {
		assert!(!target::get_sign(1.0));
		assert!(target::get_sign(-1.0));
	}

	#[test]
	fn get_exponent() {
		assert_eq!(target::get_exponent(1.0).unwrap(), 0);
		assert_eq!(target::get_exponent(2.0).unwrap(), 1);
		assert_eq!(target::get_exponent(4.0).unwrap(), 2);
		assert_eq!(target::get_exponent(8.0).unwrap(), 3);
		assert_eq!(target::get_exponent(0.5).unwrap(), -1);
		assert_eq!(target::get_exponent(0.25).unwrap(), -2);
		assert_eq!(target::get_exponent(0.125).unwrap(), -3);
		assert_eq!(target::get_exponent(0.0).unwrap(), 0);

		assert!(target::get_exponent(f32::INFINITY).is_none());
		assert!(target::get_exponent(f32::NEG_INFINITY).is_none());
		assert!(target::get_exponent(f32::NAN).is_none());
		assert!(target::get_exponent(f32::NEG_NAN).is_none());
	}
}
