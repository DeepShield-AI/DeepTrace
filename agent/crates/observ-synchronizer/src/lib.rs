use crate::config::config_listener;
use log::{info, warn};
use observ_core::Module;
use observ_runtime::{block_on, spawn_blocking};
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use tokio::task::JoinHandle;
mod config;
mod error;
mod statistic;
mod stats;
pub use error::SynchronizerError;
pub struct Synchronizer {
	running: Arc<AtomicBool>,
	handles: Option<Vec<JoinHandle<()>>>,
}

impl Synchronizer {
	pub fn new() -> Self {
		Self { running: Default::default(), handles: None }
	}
}

impl Module for Synchronizer {
	type Config = ();

	type Output = ();

	type Error = SynchronizerError;
	fn name(&self) -> &str {
		"Synchronizer"
	}
	fn start(&mut self) -> Result<(), Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("{} sender is already running.", self.name());
			return Ok(());
		}
		info!("Starting {} module...", self.name());
		let running = Arc::clone(&self.running);
		let mut handles = vec![];
		let config_listener = spawn_blocking(|| {
			block_on(async {
				let _ = config_listener().launch().await;
			})
		});
		let health_checker = spawn_blocking(|| {
			block_on(async {
				stats::health_checker(running).await;
			})
		});
		handles.push(config_listener);
		handles.push(health_checker);
		self.handles = Some(handles);

		info!("{} module started", self.name());
		Ok(())
	}
	async fn stop(&mut self) -> Result<(), Self::Error> {
		if let Some(handles) = self.handles.take() {
			for handle in handles {
				if !handle.is_finished() {
					info!("Waiting for {} module to stop...", self.name());
					handle.abort();
					// handle.await.unwrap();
				}
			}
		}
		info!("{} module stopped.", self.name());
		Ok(())
	}
}
