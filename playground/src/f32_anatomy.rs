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

pub fn extract_sign(value: f32) -> bool {
	let tmp = value.to_bits();
	(tmp & SIGN_MASK) != 0
}

pub fn extract_exponent(value: f32) -> i8 {
	let tmp = value.to_bits();
	let tmp = ((tmp & EXPONENT_MASK) >> EXPONENT_SHIFT) as i8;
	dbg!(tmp);
	if tmp == 0 { 
		if extract_mantissa(value) == 0 { 0 } else { -127 }
	} else { tmp.wrapping_sub(127)  }
}

pub fn extract_mantissa(value: f32) -> u32 {
	todo!("not implemented");
}

pub fn mantissa_to_array(value: f32) -> [bool; 24] {
	todo!("not implemented");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extract_sign() {
		assert_eq!(extract_sign(1.0), false);
		assert_eq!(extract_sign(-1.0), true);
	}

	#[test]
	fn test_extract_exponent() {
		assert_eq!(extract_exponent(1.0), 0);
		assert_eq!(extract_exponent(2.0), 1);
		assert_eq!(extract_exponent(4.0), 2);
		assert_eq!(extract_exponent(8.0), 3);
		assert_eq!(extract_exponent(0.5), -1);
		assert_eq!(extract_exponent(0.25), -2);
		assert_eq!(extract_exponent(0.125), -3);
		assert_eq!(extract_exponent(0.0), 0);
	}

	#[test]
	fn test_extract_mantissa() {
		assert_eq!(extract_mantissa(1.0), 0x00_80_00_00);
	}
}
