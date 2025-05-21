use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to initialize trace module")]
	InitError,
	#[error("Failed to config")]
	Config(#[from] config::ConfigError),
}
