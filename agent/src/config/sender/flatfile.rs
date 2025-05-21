use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub mem_buffer_size: usize,
	pub file_buffer_size: usize,
	pub file_size_limit: usize,
}
