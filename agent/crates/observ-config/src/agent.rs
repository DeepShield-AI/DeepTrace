use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AgentConfig {
	name: String,
	user: String,
}
