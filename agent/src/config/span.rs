use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub batch_size: usize,
}
