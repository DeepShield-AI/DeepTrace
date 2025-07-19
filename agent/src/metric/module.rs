use super::{CpuCollector, CpuLogger, MetricError};
use crate::{Module, app::runtime::spawn};
use log::{info, warn};
use std::{
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};
use tokio::task::JoinHandle;

pub struct MetricCollector {
	cpu_collector_handle: Option<JoinHandle<()>>,
	cpu_collector_running: Arc<AtomicBool>,
	logger: Option<Arc<Mutex<CpuLogger>>>,
}

impl MetricCollector {
	pub fn new() -> Result<Self, MetricError> {
		let logger = match CpuLogger::new("cpu_usage.csv") {
			Ok(logger) => Some(Arc::new(Mutex::new(logger))),
			Err(e) => {
				warn!("Failed to create CPU usage logger: {}", e);
				None
			},
		};

		Ok(Self {
			cpu_collector_handle: None,
			cpu_collector_running: Arc::new(AtomicBool::new(false)),
			logger,
		})
	}

	fn start_cpu_collector(&mut self) {
		let running = Arc::clone(&self.cpu_collector_running);

		let logger = match self.logger.as_ref() {
			Some(logger) => Arc::clone(logger),
			None => {
				warn!("Logger not initialized");
				return;
			},
		};

		self.cpu_collector_handle = Some(spawn(async move {
			let collector = CpuCollector::new();
			info!("CPU usage collector started");

			while running.load(Ordering::Relaxed) {
				let usages = collector.collect();
				for usage in &usages {
					info!("CPU {}: {:.2}%", usage.cpu_id, usage.cpu_usage);
					let logger = logger.lock().unwrap();
					logger.write(usage);
				}
				collector.sleep_duration().await;
			}

			let logger = logger.lock().unwrap();
			logger.flush();
			info!("CPU usage collector stopped");
		}));
	}
}

impl Module for MetricCollector {
	type Error = MetricError;

	fn name(&self) -> &str {
		"Metric Collector"
	}

	fn start(&mut self) -> Result<(), Self::Error> {
		info!("Starting {}", self.name());

		if self.cpu_collector_running.swap(true, Ordering::Relaxed) {
			warn!("CPU collector is already running");
			return Ok(());
		}
		self.start_cpu_collector();

		info!("{} started", self.name());
		Ok(())
	}

	async fn stop(&mut self) -> Result<(), Self::Error> {
		info!("Stopping {}", self.name());
		if !self.cpu_collector_running.swap(false, Ordering::Relaxed) {
			warn!("CPU collector is not running");
			return Ok(());
		}
		if let Some(handle) = self.cpu_collector_handle.take() {
			if !handle.is_finished() {
				info!("Waiting for CPU collector to finish...");
				handle.await.expect("Failed to stop CPU collector");
			}
		}

		info!("{} stopped", self.name());
		Ok(())
	}
}
