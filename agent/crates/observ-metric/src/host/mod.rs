use super::{Collector, MetricError};
use cpu::CpuCollector;
use disk::DiskCollector;
use log::warn;
use memory::MemoryCollector;
use network::NetworkCollector;
use observ_event::metric::Metric;

mod constants;
mod cpu;
mod disk;
mod memory;
mod network;

pub(super) struct HostCollector {
	collectors: Vec<Box<dyn Collector>>,
}

impl HostCollector {
	pub fn new() -> Result<Self, MetricError> {
		let mut collectors: Vec<Box<dyn Collector>> = Vec::new();
		let cpu_collector = CpuCollector::new()?;
		let memory_collector = MemoryCollector::new()?;
		let network_collector = NetworkCollector::new()?;
		let disk_collector = DiskCollector::new()?;
		collectors.push(Box::new(cpu_collector));
		collectors.push(Box::new(memory_collector));
		collectors.push(Box::new(network_collector));
		collectors.push(Box::new(disk_collector));
		Ok(Self { collectors })
	}
}

#[async_trait::async_trait]
impl Collector for HostCollector {
	fn name(&self) -> &'static str {
		"Host Collector"
	}

	async fn collect(&self, buffer: &mut Vec<Metric>) -> Result<(), MetricError> {
		for collector in &self.collectors {
			if let Err(e) = collector.collect(buffer).await {
				warn!("Collector {} failed: {:?}", collector.name(), e);
			}
		}
		Ok(())
	}
}
