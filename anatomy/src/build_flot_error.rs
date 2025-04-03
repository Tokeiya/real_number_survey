use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildFlotError<T> {
	#[error("MantissaError: {0}")]
	MantissaError(T),
	#[error("ExponentError: {0}")]
	ExponentError(T),
}
