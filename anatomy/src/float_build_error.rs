use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FloatBuildError<T: Debug, U: Debug> {
	#[error("Invalid exponent value: {0}")]
	InvalidExponentValue(T),
	#[error("Invalid mantissa value: {0}")]
	InvalidMantissaValue(U),
}
