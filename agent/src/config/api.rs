use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
	pub address: String,
	pub port: u16,
	pub workers: usize,
	pub ident: String,
}
