use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ElasticSenderConfig {
	/// node URL
	pub node_url: String,
	pub username: String,
	pub password: String,
	/// request timeout (seconds)
	pub request_timeout: u64,
	pub index_name: String,
	/// bulk size
	pub bulk_size: usize,
}
