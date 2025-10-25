use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SpanConfig {
	pub cleanup_interval: u64,
	pub max_sockets: usize,
}
