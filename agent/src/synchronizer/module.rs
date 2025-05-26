use super::{SynchronizerError, config_listener};
use crate::{
	Module,
	app::runtime::{block_on, spawn_blocking},
	synchronizer::state,
};
use log::info;
use tokio::task::JoinHandle;
pub(crate) struct Synchronizer {
	handles: Option<Vec<JoinHandle<()>>>,
}

impl Synchronizer {
	pub fn new() -> Self {
		Self { handles: None }
	}
}

impl Module for Synchronizer {
	type Error = SynchronizerError;
	fn name(&self) -> &str {
		"Synchronizer"
	}
	fn start(&mut self) -> Result<(), Self::Error> {
		info!("Starting {} module...", self.name());

		let mut handles = vec![];
		let config_listener = spawn_blocking(|| {
			block_on(async {
				let _ = config_listener().launch().await;
			})
		});
		let state_checker = spawn_blocking(|| {
			block_on(async {
				state::health_checker().await;
			})
		});
		handles.push(config_listener);
		handles.push(state_checker);
		self.handles = Some(handles);

		info!("{} module started", self.name());
		Ok(())
	}
	async fn stop(&mut self) -> Result<(), Self::Error> {
		if let Some(handles) = self.handles.take() {
			for handle in handles {
				if !handle.is_finished() {
					info!("Waiting for {} module to stop...", self.name());
					handle.await.unwrap();
				}
			}
		}
		info!("{} module stopped.", self.name());
		Ok(())
	}
}
