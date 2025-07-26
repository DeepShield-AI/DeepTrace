use super::{
	CpuCollectorManager, DiskCollectorManager, MemCollectorManager, MetricError,
	NetCollectorManager,
};
use crate::{
	Module,
	app::runtime::spawn,
	metric::{CpuLogger, DiskLogger, MemLogger, NetLogger},
};
use log::{info, warn};
use std::sync::{
	Arc, Mutex,
	atomic::{AtomicBool, Ordering},
};
pub struct MetricCollector {
	cpu_collector: Option<CpuCollectorManager>,
	disk_collector: Option<DiskCollectorManager>,
	mem_collector: Option<MemCollectorManager>,
	net_collector: Option<NetCollectorManager>, // Add NetCollectorManager
}
impl MetricCollector {
	pub fn new() -> Result<Self, MetricError> {
		let cpu_collector: Option<CpuCollectorManager> = match CpuCollectorManager::new(Some(
			Arc::new(Mutex::new(CpuLogger::new("cpu_usage.csv")?)),
		)) {
			manager => Some(manager),
		};
		let disk_logger = Some(Arc::new(Mutex::new(DiskLogger::new("disk")?)));
		let disk_collector = Some(DiskCollectorManager::new(disk_logger));

		let mem_logger = Some(Arc::new(Mutex::new(MemLogger::new("mem_usage.csv")?)));
		let mem_collector = Some(MemCollectorManager::new(mem_logger));
		let net_logger = Some(Arc::new(Mutex::new(NetLogger::new("net_usage.csv")?)));
		let net_collector = Some(NetCollectorManager::new(net_logger));
		Ok(Self { cpu_collector, disk_collector, mem_collector, net_collector })
	}
}

impl Module for MetricCollector {
	type Error = MetricError; // 定义 Error 类型

	fn name(&self) -> &str {
		// 实现 name 方法
		"Metric Collector"
	}
	fn start(&mut self) -> Result<(), Self::Error> {
		info!("Starting Metric Collector");

		if let Some(ref mut manager) = self.cpu_collector {
			manager.start_collector();
		} else {
			warn!("CPU collector not initialized");
		}

		if let Some(ref mut manager) = self.disk_collector {
			manager.start_collector();
		} else {
			warn!("Disk collector not initialized");
		}

		if let Some(ref mut manager) = self.mem_collector {
			manager.start_collector();
		} else {
			warn!("Mem collector not initialized");
		}

		if let Some(ref mut manager) = self.net_collector {
			manager.start_collector();
		} else {
			warn!("Net collector not initialized");
		}
		info!("Metric Collector started");
		Ok(())
	}

	async fn stop(&mut self) -> Result<(), Self::Error> {
		info!("Stopping Metric Collector");

		if let Some(ref mut manager) = self.cpu_collector {
			manager.stop_collector().await;
		}

		if let Some(ref mut manager) = self.disk_collector {
			manager.stop_collector().await;
		}

		if let Some(ref mut manager) = self.mem_collector {
			manager.stop_collector().await;
		}

		if let Some(ref mut manager) = self.net_collector {
			manager.stop_collector().await;
		}

		info!("Metric Collector stopped");
		Ok(())
	}
}
