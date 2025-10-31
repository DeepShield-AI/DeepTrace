// use crate::{app::Context, config, metric, sender, synchronizer, trace};
use std::io;
use thiserror::Error;
use tokio::sync::SetError;

#[derive(Debug, Error)]
pub enum AgentError {
	#[error("Config module error: {0}")]
	Config(#[from] observ_config::ConfigError),
	#[error("Trace module error: {0}")]
	Trace(#[from] observ_trace::TraceError),
	#[error("Sender module error: {0}")]
	Sender(#[from] observ_sender::SendError),
	#[error("Elastic sender error: {0}")]
	ElasticSender(#[from] observ_sender::elastic::ElasticError),
}
