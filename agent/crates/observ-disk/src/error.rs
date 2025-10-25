use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiskMetricError {
	#[error("IO error: {0}")]
	IoError(#[from] io::Error),
	#[error("Parse error: {0}")]
	ParseError(#[from] Box<dyn std::error::Error>),
}
