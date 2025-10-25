use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryMetricError {
	#[error("IO operated error: {0}")]
	IOError(#[from] io::Error),
	#[error("Parse error: {0}")]
	ParseError(#[from] Box<dyn std::error::Error>),
}
