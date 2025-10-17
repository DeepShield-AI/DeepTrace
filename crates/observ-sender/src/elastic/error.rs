use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElasticError {
	// #[error("Failed to serialize JSON: {0}")]
	// Json(#[from] serde_json::Error),
	#[error("Failed to parse URL")]
	ParseUrl,
	#[error("Failed to build transport: {0}")]
	Build(#[from] elasticsearch::http::transport::BuildError),
	#[error("Failed to send request: {0}")]
	Request(#[from] elasticsearch::Error),
	#[error("Failed to parse response: {0}")]
	Response(String),
}
