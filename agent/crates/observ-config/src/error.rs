use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
	#[error("config file not found")]
	FileNotFound,
	#[error("failed to set config")]
	SetConfigFailed,
	#[error("Config error occurred: {0}")]
	ConfigError(#[from] config::ConfigError),
	#[error("Notify error occurred: {:?}, paths: {}", .0.kind, .0.paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", "))]
	NotifyError(#[from] notify::Error),
}
