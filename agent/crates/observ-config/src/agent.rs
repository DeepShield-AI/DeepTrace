use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AgentConfig {
	pub name: String,
	pub user: String,
}
