use super::encode::CodecEncodeError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum CodecError {
	#[error("Encode error: {0}")]
	Encode(#[from] CodecEncodeError),
}
