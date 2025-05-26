use elasticsearch::http::transport;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to parse URL: {0}")]
	Url(#[from] url::ParseError),
	#[error("Failed to build Elasticsearch client: {0}")]
	Build(#[from] transport::BuildError),
	#[error("Failed to send request: {0}")]
	Request(#[from] elasticsearch::Error),
	#[error("Failed to parse response")]
	Response,
}
