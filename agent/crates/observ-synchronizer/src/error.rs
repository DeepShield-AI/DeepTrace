use elasticsearch::http::transport;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SynchronizerError {
	#[error("Failed to parse URL")]
	ParseUrl,
	#[error("Failed to build Elasticsearch client: {0}")]
	Build(#[from] transport::BuildError),
	#[error("Failed to send request: {0}")]
	Request(#[from] elasticsearch::Error),
	#[error("Failed to parse response")]
	Response,
}
