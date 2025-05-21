use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Io operation failed: {0}")]
	IO(#[from] io::Error),
	#[error("Failed to serialize JSON: {0}")]
	Json(#[from] serde_json::Error),
}
