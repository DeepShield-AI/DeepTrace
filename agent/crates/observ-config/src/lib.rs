pub use access::*;
pub use agent::AgentConfig;
use arc_swap::ArcSwap;
use config::{Config, File};
pub use ebpf::EbpfConfig;
pub use error::ConfigError;
use log::{debug, error, info, warn};
pub use metric::MetricConfig;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use observ_core::Module;
use observ_runtime::handle;
use rustc_hash::FxHashMap;
pub use sender::{ElasticSenderConfig, SenderConfig};
use serde::Deserialize;
use std::{
	path::{Path, PathBuf},
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
		mpsc::channel,
	},
	time::Duration,
};
use tokio::{sync::OnceCell, task::JoinHandle, time::sleep};
pub use trace::TraceConfig;

mod access;
mod agent;
mod ebpf;
mod error;
mod metric;
mod sender;
mod trace;

static CONFIG: OnceCell<Arc<ArcSwap<ObservConfig>>> = OnceCell::const_new();

#[derive(Debug, Deserialize)]
pub struct ObservConfig {
	pub(crate) agent: AgentConfig,
	pub(crate) sender: Option<SenderConfig>,
	pub(crate) metric: Option<MetricConfig>,
	pub(crate) trace: Option<TraceConfig>,
	pub(crate) ebpf: Option<FxHashMap<String, EbpfConfig>>,
}

impl ObservConfig {
	pub(crate) fn load(path: impl AsRef<str>) -> Result<Self, ConfigError> {
		Self::load_from_file(path.as_ref()).inspect_err(|e| {
			warn!("Failed to load config from {}: {e}, using default", path.as_ref())
		})
	}

	fn load_from_file(path: impl AsRef<str>) -> Result<Self, ConfigError> {
		let config = Config::builder().add_source(File::with_name(path.as_ref())).build()?;
		Ok(config.try_deserialize::<ObservConfig>()?)
	}

	/// Load config with retry logic to handle file write delays
	async fn load_from_file_with_retry_async(
		path: impl AsRef<str>,
		max_retries: usize,
	) -> Result<Self, ConfigError> {
		let path_str = path.as_ref();
		let mut last_error = None;

		for attempt in 0..=max_retries {
			// Check if file exists and is not empty
			match std::fs::read_to_string(path_str) {
				Ok(content) if !content.trim().is_empty() => {
					// File has content, try to parse it
					match Self::load_from_file(path_str) {
						Ok(config) => return Ok(config),
						Err(e) => {
							last_error = Some(e);
							if attempt < max_retries {
								// Wait before retry with exponential backoff
								sleep(Duration::from_millis(50 * (1 << attempt))).await;
								continue;
							}
						},
					}
				},
				Ok(_) => {
					info!("Config file is empty, retrying...");
					// File is empty, wait and retry
					if attempt < max_retries {
						sleep(Duration::from_millis(50 * (1 << attempt))).await;
						continue;
					} else {
						last_error = Some(ConfigError::ConfigError(config::ConfigError::Message(
							"Config file is empty after all retries".to_string(),
						)));
					}
				},
				Err(e) => {
					last_error = Some(ConfigError::ConfigError(config::ConfigError::Message(
						format!("Failed to read config file: {}", e),
					)));
					if attempt < max_retries {
						sleep(Duration::from_millis(50 * (1 << attempt))).await;
						continue;
					}
				},
			}
		}

		Err(last_error.unwrap_or_else(|| {
			ConfigError::ConfigError(config::ConfigError::Message(
				"Failed to load config after all retries".to_string(),
			))
		}))
	}
}

pub fn config() -> Arc<ArcSwap<ObservConfig>> {
	Arc::<ArcSwap<ObservConfig>>::clone(CONFIG.get().expect("Config is not initialized"))
}

pub fn change_config(config: ObservConfig) {
	CONFIG.get().expect("Config is not initialized").store(Arc::new(config));
	info!("Configuration updated successfully");
	debug!("New configuration: {:?}", CONFIG.get().unwrap().load());
}

pub struct Configurator {
	path: String,
	running: Arc<AtomicBool>,
	handle: Option<JoinHandle<Result<(), ConfigError>>>,
}

impl Configurator {
	pub fn new(path: String) -> Result<Self, ConfigError> {
		let config_path = PathBuf::from(&path);
		if !config_path.exists() {
			return Err(config::ConfigError::NotFound(path).into());
		}
		let config = ObservConfig::load(&path)?;

		CONFIG
			.set(Arc::new(ArcSwap::from_pointee(config)))
			.inspect_err(|_| error!("Failed to set config."))
			.map_err(|_| ConfigError::SetConfigFailed)?;

		Ok(Self { path, running: Default::default(), handle: None })
	}
}

impl Module for Configurator {
	type Config = ();
	type Error = ConfigError;
	type Output = ();

	fn name(&self) -> &str {
		"Observ Configurator"
	}

	fn start(&mut self) -> Result<(), Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("Observ Configurator is already running.");
		}
		let config_path = self.path.clone();
		self.handle = Some(handle().spawn(async move {
			let (tx, rx) = channel();

			// Automatically select the best implementation for your platform.
			// You can also access each implementation directly e.g. INotifyWatcher.
			let mut watcher: RecommendedWatcher = Watcher::new(
				tx,
				// TODO: make this configurable?
				notify::Config::default().with_poll_interval(Duration::from_secs(1)),
			)?;

			// Add a path to be watched. All files and directories at that path and
			// below will be monitored for changes.
			watcher.watch(Path::new(&config_path), RecursiveMode::NonRecursive)?;

			// This is a simple loop, but you may want to use more complex logic here,
			// for example to handle I/O.
			loop {
				match rx.recv() {
					Ok(Ok(Event { kind: notify::event::EventKind::Modify(_), .. })) => {
						debug!("{} written; refreshing configuration ...", config_path);

						// Use retry logic to handle file write delays
						match ObservConfig::load_from_file_with_retry_async(&config_path, 3).await {
							Ok(config) => {
								change_config(config);
							},
							Err(e) => {
								error!(
									"Failed to load config after modification with retries: {}",
									e
								);
								// Don't panic, just log the error and continue
							},
						}
						sleep(Duration::from_millis(50)).await;
					},

					Err(e) => error!("watch error: {e:?}"),

					_ => {
						// Ignore event
					},
				}
			}
		}));
		Ok(())
	}

	async fn stop(&mut self) -> Result<Self::Output, Self::Error> {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("observ configurator is already stopped.");
			return Ok(());
		}

		if let Some(thread) = self.handle.take() {
			thread.abort();
			// .await
			// .unwrap_or_else(|_| panic!("Failed to join observ configurator"))?;
		}
		info!("observ configurator stopped.");
		Ok(())
	}
}
