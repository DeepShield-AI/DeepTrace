use super::backend::{ElasticError, FlatFileError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SendError {
	#[error("Failed to send to flat file: {0}")]
	FlatFile(#[from] FlatFileError),
	#[error("Failed to send to elastic: {0}")]
	Elasticsearch(#[from] ElasticError),
}

unsafe impl Send for SendError {}
unsafe impl Sync for SendError {}
