use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodecEncodeError {
	#[error("JSON encode error: {0}")]
	Json(#[from] serde_json::Error),
	#[error("IO error: {0}")]
	IO(#[from] std::io::Error),
}
