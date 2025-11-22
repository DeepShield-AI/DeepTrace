use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SynchronizerConfig {
	pub sender: String,
}
