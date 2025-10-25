use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CpuMetricError {
	#[error("IO error: {0}")]
	IOError(#[from] io::Error),
	#[error("ProcParser error: {0}")]
	ProcParserError(#[from] Box<dyn std::error::Error>),
}
