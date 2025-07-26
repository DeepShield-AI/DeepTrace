use crate::{
	Module,
	app::runtime::{block_on, spawn, spawn_blocking},
	metric::{DiskCollector, DiskLogger},
};
use log::{info, warn};
use std::{
	collections::HashMap,
	sync::{
		Arc, Mutex,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};
use tokio::task::JoinHandle;

pub struct DiskCollectorManager {
	handle: Option<JoinHandle<()>>,
	running: Arc<AtomicBool>,
	logger: Option<Arc<Mutex<DiskLogger>>>,
}

impl DiskCollectorManager {
	pub fn new(logger: Option<Arc<Mutex<DiskLogger>>>) -> Self {
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

		let mut collector = DiskCollector::new();

		self.handle = Some(spawn_blocking(move || {
			block_on(async move {
				while running.load(Ordering::Relaxed) {
					// 收集指标
					let mut metrics = collector.collect_metrics();
					let usages = collector.collect_usages();
					let ext4_cache_stats = collector.collect_ext4_cache_stats();

					// 写入日志
					let mut logger = logger.lock().unwrap();
					logger.write_metrics(&metrics);
					logger.write_usages(&usages);

					match ext4_cache_stats {
						Ok(stats) => logger.write_ext4_cache(&stats),
						Err(e) => warn!("Failed to collect ext4 cache stats: {}", e),
					}

					// 等待下一次采集
					collector.sleep_duration().await;
				}

				// 刷新日志
				let mut logger = logger.lock().unwrap();
				logger.flush();
				info!("Disk metrics collector stopped");
			});
		}));
	}

	pub async fn stop_collector(&mut self) {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("Disk collector is not running");
			return;
		}

		if let Some(handle) = self.handle.take() {
			if !handle.is_finished() {
				info!("Waiting for disk collector to finish...");
				handle.await.expect("Failed to stop disk collector");
			}
		}
	}
}
