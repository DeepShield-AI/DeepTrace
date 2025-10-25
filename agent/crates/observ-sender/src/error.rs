use crate::elastic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SendError {
	#[error("IO error: {0}")]
	IO(#[from] std::io::Error),
	#[error("Encode error: {0}")]
	Encode(#[from] codec::encode::CodecEncodeError),
	#[error("Elastic error: {0}")]
	Elastic(#[from] elastic::ElasticError),
}
