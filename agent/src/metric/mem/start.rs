// mem/start.rs

use crate::{
	Module,
	app::runtime::{block_on, spawn, spawn_blocking},
	metric::{MemCollector, MemLogger},
};
use log::{info, warn};
use std::{
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};
use tokio::task::JoinHandle;

pub struct MemCollectorManager {
	handle: Option<JoinHandle<()>>,
	running: Arc<AtomicBool>,
	logger: Option<Arc<Mutex<MemLogger>>>,
}

impl MemCollectorManager {
	pub fn new(logger: Option<Arc<Mutex<MemLogger>>>) -> Self {
		Self { handle: None, running: Arc::new(AtomicBool::new(false)), logger }
	}

	pub fn start_collector(&mut self) {
		if self.running.swap(true, Ordering::Relaxed) {
			return;
		}

		let running = Arc::clone(&self.running);
		let logger = match self.logger.as_ref() {
			Some(logger) => Arc::clone(logger),
			None => {
				warn!("Logger not initialized");
				return;
			},
		};

		self.handle = Some(spawn_blocking(move || {
			block_on(async move {
				let collector = MemCollector::new();
				while running.load(Ordering::Relaxed) {
					let metric = collector.collect();
					if let Ok(mut logger) = logger.lock() {
						for metric1 in metric {
							logger.write(&metric1);
						}
					}
					collector.sleep_duration().await;
				}

				if let Ok(mut logger) = logger.lock() {
					logger.flush();
				}
				info!("Memory usage collector stopped");
			})
		}));
	}

	pub async fn stop_collector(&mut self) {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("Memory collector is not running");
			return;
		}

		if let Some(handle) = self.handle.take() {
			if !handle.is_finished() {
				info!("Waiting for Memory collector to finish...");
				handle.await.expect("Failed to stop Memory collector");
			}
		}
	}
}
