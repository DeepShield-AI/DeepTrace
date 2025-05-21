use super::{AgentConfig, ApiConfig, ConfigError, SenderConfig, TraceConfig};
use crate::constants::DEFAULE_CONFIG_PATH;
use config::{Config, File};
use log::{error, warn};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
	pub api: ApiConfig,
	pub agent: AgentConfig,
	pub sender: SenderConfig,
	pub ebpf: TraceConfig,
}

impl AppConfig {
	pub fn load(path: impl AsRef<str>) -> Result<Self, ConfigError> {
		Self::load_from_file(path.as_ref())
			.inspect_err(|e| {
				warn!("Failed to load config from {}: {}, using default", path.as_ref(), e)
			})
			.or_else(|_| Self::load_default_config())
	}

	fn load_from_file(path: impl AsRef<str>) -> Result<Self, ConfigError> {
		let config = Config::builder().add_source(File::with_name(path.as_ref())).build()?;
		Ok(config.try_deserialize::<AppConfig>()?)
	}

	fn load_default_config() -> Result<Self, ConfigError> {
		Self::load_from_file(DEFAULE_CONFIG_PATH)
			.inspect_err(|e| error!("Failed to load default config: {}", e))
	}
}
