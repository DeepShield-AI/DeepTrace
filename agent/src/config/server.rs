use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub ip: String,
	pub port: u16,
	pub path: String,
}
