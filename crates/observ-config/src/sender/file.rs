use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct FileSenderConfig {
	/// file path
	pub path: PathBuf,
	/// whether to enable log rotation
	pub rotate: bool,
	/// maximum file size (MB)
	#[serde(rename = "max_size")]
	pub max_size_mb: usize,

	/// maximum retention days
	#[serde(rename = "max_age")]
	pub max_age_days: u32,

	/// rotate interval days
	#[serde(rename = "rotate_time")]
	pub rotate_time_days: u32,

	/// date format
	pub data_format: String,
}

impl Default for FileSenderConfig {
	fn default() -> Self {
		Self {
			path: PathBuf::from("./logs"),
			rotate: false,
			max_size_mb: 100,
			max_age_days: 30,
			rotate_time_days: 1,
			data_format: "%Y-%m-%d".to_string(),
		}
	}
}
