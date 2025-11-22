use thiserror::Error;

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
	#[error("Span constructor error: {0}")]
	SpanConstructor(#[from] observ_trace::span::SpanError),
	#[error("Synchronizer module error: {0}")]
	Synchronizer(#[from] observ_synchronizer::SynchronizerError),
}
