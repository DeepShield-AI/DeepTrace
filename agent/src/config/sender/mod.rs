use serde::Deserialize;

mod elastic;
pub(crate) use elastic::Config as ElasticConfig;
mod flatfile;
pub use flatfile::Config as FlatFileConfig;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Config {
	pub batch_size: usize,
	pub elastic: ElasticConfig,
	pub flat_file: FlatFileConfig,
}
