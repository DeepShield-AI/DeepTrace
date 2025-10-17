use std::{convert, io, num};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetricError {
	#[error("IO operated error: {0}")]
	IOError(#[from] io::Error),
	#[error("Failed to parse string: {0}")]
	ParseStringError(#[from] convert::Infallible),
	#[error("Parse float error: {0}")]
	ParseFloatError(#[from] num::ParseFloatError),
	#[error("Parse int error: {0}")]
	ParseIntError(#[from] num::ParseIntError),
	#[error("Missing key '{0}' in file '{1}'")]
	ParseKeyError(String, String),
	#[error("Failed to init collectors.")]
	Init,
	#[error("Failed to send metrics.")]
	Send,
}
