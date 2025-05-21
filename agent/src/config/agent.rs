use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub workers: usize,
	// pub channel_size: usize,
}
