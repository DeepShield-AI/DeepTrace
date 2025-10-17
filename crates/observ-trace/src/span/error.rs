use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpanError {
	#[error("missing receiver")]
	MissingReceiver,
	#[error("missing sender")]
	MissingSender,
}
