use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Config {
	pub node_url: String,
	pub username: String,
	pub password: String,
	pub request_timeout: u64,
	pub index_name: String,
	pub bulk_size: usize,
}
