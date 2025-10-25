pub use elastic::ElasticSenderConfig;
pub use file::FileSenderConfig;
use rustc_hash::FxHashMap;
use serde::Deserialize;

mod elastic;
mod file;

#[derive(Debug, Deserialize)]
pub struct SenderConfig {
	pub(crate) elastic: Option<FxHashMap<String, ElasticSenderConfig>>,
	pub(crate) file: Option<FxHashMap<String, FileSenderConfig>>,
}
