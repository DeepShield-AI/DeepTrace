use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EbpfConfig {
	pub log_level: u32,
	pub enabled_probes: Vec<String>,
	pub max_buffered_events: u16,
	pub pids: Vec<u32>,
}
