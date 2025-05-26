use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub pids: Vec<u32>,
}
