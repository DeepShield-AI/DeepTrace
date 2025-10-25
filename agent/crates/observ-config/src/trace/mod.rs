use serde::Deserialize;
pub use span::SpanConfig;
mod span;

#[derive(Deserialize, Debug)]
pub struct TraceConfig {
	pub name: String,
	pub span: SpanConfig,
}
