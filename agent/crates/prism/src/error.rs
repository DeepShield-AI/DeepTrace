use thiserror::Error;
#[derive(Error, Debug)]
pub enum AgentError {
	#[error("Metric error: {0}")]
	MetricError(#[from] observ_metric::MetricError),
	#[error("Send error: {0}")]
	SendError(#[from] observ_sender::SendError),
	#[error("Config error: {0}")]
	ConfigError(#[from] observ_config::ConfigError),
}
