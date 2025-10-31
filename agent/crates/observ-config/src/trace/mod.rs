use serde::Deserialize;
pub use span::SpanConfig;
mod span;

#[derive(Deserialize, Debug)]
pub struct TraceConfig {
	pub ebpf: String,
	pub sender: String,
	pub span: SpanConfig,
}
