// use std::thread::{self, JoinHandle};

use super::ConfigError;
use crate::{
	AgentError, Module,
	app::{
		config_listener,
		runtime::block_on,
	}
};
use log::info;
use std::thread::{self, JoinHandle};

pub struct ConfigModule {
	// config: ConfigAccess,
	handle: Option<JoinHandle<()>>,
}

impl ConfigModule {
	pub fn new() -> Self {
		Self { handle: None }
	}
}

impl Module for ConfigModule {
	type Error = ConfigError;
	fn name(&self) -> &str {
		"Config"
	}
	fn start(&mut self) -> Result<(), Self::Error> {
		info!("Starting {} module...", self.name());

		self.handle = Some(
			thread::Builder::new()
				.name("config-listener".to_owned())
				.spawn(|| {
					block_on(async {
						let _start = config_listener().launch().await;
					});
				})
				.expect("Failed to spawn config listener thread"),
		);
		Ok(())
	}
	fn stop(&mut self) -> Result<(), Self::Error> {
		if let Some(handle) = self.handle.take() {
			if !handle.is_finished() {
				info!("Waiting for {} module to stop...", self.name());
				handle.join().unwrap();
			}
		}
		info!("{} module stopped.", self.name());
		Ok(())
	}
}
