use crossbeam_channel::Sender;
pub use error::MetricError;
use host::HostCollector;
use log::{info, warn};
use observ_core::Module;
use observ_event::metric::Metric;
use observ_metric_common::init_roots;
use observ_runtime::handle;
use std::{
	mem,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};
use tokio::{task::JoinHandle, time};
use tokio_stream::{StreamExt, wrappers::IntervalStream};
mod error;
mod host;

#[async_trait::async_trait]
pub(crate) trait Collector: Send + Sync {
	fn name(&self) -> &'static str;
	async fn collect(&self, buffer: &mut Vec<Metric>) -> Result<(), MetricError>;
}

pub struct MetricCollector {
	running: Arc<AtomicBool>,
	collectors: Option<Vec<Box<dyn Collector>>>,
	handle: Option<JoinHandle<Result<(), MetricError>>>,
	output: Sender<Vec<Metric>>,
}

impl MetricCollector {
	pub fn new(output: Sender<Vec<Metric>>) -> Result<Self, MetricError> {
		init_roots();
		let mut collectors: Vec<Box<dyn Collector>> = Vec::new();
		let host_collector = HostCollector::new()?;
		collectors.push(Box::new(host_collector));

		Ok(MetricCollector {
			collectors: Some(collectors),
			running: Default::default(),
			handle: None,
			output,
		})
	}
}

impl Module for MetricCollector {
	type Config = ();
	type Error = MetricError;
	type Output = ();

	fn name(&self) -> &str {
		"Metric Collector"
	}

	fn start(&mut self) -> Result<(), Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("{} sender is already running.", self.name());
			return Ok(());
		}
		let running = Arc::clone(&self.running);
		let output = self.output.clone();
		let collectors = self.collectors.take().ok_or(Self::Error::Init)?;
		self.handle = Some(handle().spawn(async move {
			let mut interval = IntervalStream::new(time::interval(Duration::from_secs(1)));
			let mut buffer = Vec::new();
			while running.load(Ordering::Relaxed) && interval.next().await.is_some() {
				for collector in &collectors {
					if let Err(e) = collector.collect(&mut buffer).await {
						warn!("Collector {} failed: {:?}", collector.name(), e);
					}
				}
				let metrics = mem::take(&mut buffer);
				output.send(metrics).map_err(|_| MetricError::Send)?;
				// for metrics in buffer.drain(..) {
				// 	if output.send(metrics).await.is_err() {
				// 		warn!("Metric channel closed");
				// 		break;
				// 	}
				// }
			}
			Ok(())
		}));
		Ok(())
	}

	async fn stop(&mut self) -> Result<Self::Output, Self::Error> {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("{} is already stopped.", self.name());
			return Ok(());
		}

		if let Some(thread) = self.handle.take() {
			thread
				.await
				.unwrap_or_else(|_| panic!("Failed to join {} thread", self.name()))?;
		}
		info!("{} stopped.", self.name());
		Ok(())
	}
}
