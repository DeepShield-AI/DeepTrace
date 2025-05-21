use elasticsearch::http::transport;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Io operation failed: {0}")]
	IO(#[from] io::Error),
	#[error("Failed to serialize JSON: {0}")]
	Json(#[from] serde_json::Error),
	#[error("Failed to parse URL: {0}")]
	Url(#[from] url::ParseError),
	#[error("Failed to build Elasticsearch client: {0}")]
	Build(#[from] transport::BuildError),
	#[error("Failed to send request: {0}")]
	Request(#[from] elasticsearch::Error),
	#[error("Failed to parse response: {0}")]
	Response(String),
}
