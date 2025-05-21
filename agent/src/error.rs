use crate::{app::Context, config, sender, trace};
use std::io;
use thiserror::Error;
use tokio::sync::SetError;

#[derive(Debug, Error)]
pub enum AgentError {
	#[error("Send error: {0}")]
	Send(#[from] sender::SendError),
	#[error("Config error: {0}")]
	Config(#[from] config::ConfigError),
	#[error("Runtime error: {0}")]
	Runtime(#[from] io::Error),
	#[error("Set context error: {0}")]
	Initialize(#[from] SetError<Context>),
	#[error("Trace error: {0}")]
	Trace(#[from] trace::TraceError),
	#[error("Span error: {0}")]
	Span(#[from] trace::SpanError),
}
