use arc_swap::access::Access;
use aya::{
	Ebpf,
	maps::{PerfEventArray, perf::Events},
	util::online_cpus,
};
use bytes::BytesMut;
use crossbeam_channel::Sender;
use ebpf_manager::utils::{optimal_page_count, unlock_memory};
pub use error::TraceError;
use log::{debug, error, info, warn};
use observ_config::{TraceAccess, TraceConfig, ebpf_config, trace_config};
use observ_core::Module;
use observ_runtime::{block_on, spawn_blocking};
use observ_trace_common::{Message, maps::EVENT_MAP};
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use tokio::task::JoinHandle;

mod ebpf;
mod error;
pub mod span;

type Result<T> = std::result::Result<T, TraceError>;

pub struct TraceCollector {
	config: TraceAccess,
	ebpf: Ebpf,
	output: Sender<Message>,
	running: Arc<AtomicBool>,
	handles: Option<Vec<JoinHandle<Result<()>>>>,
}

impl TraceCollector {
	pub fn new(output: Sender<Message>) -> Result<Self> {
		info!("Initializing Trace Collector");
		let config = trace_config();
		unlock_memory();
		let ebpf = ebpf::prepare_ebpf()?;
		Ok(Self { config, ebpf, output, running: Arc::new(AtomicBool::new(false)), handles: None })
	}
}

// TODO: this only collect ebpf messages, so is it appropriate to name it TraceCollector?
impl Module for TraceCollector {
	type Error = TraceError;

	type Config = TraceConfig;

	type Output = ();
	fn name(&self) -> &'static str {
		"Trace Collector"
	}

	fn start(&mut self) -> Result<()> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("{} is already running.", self.name());
			return Ok(());
		}

		info!("Starting {} module...", self.name());

		let config = self.config.load();
		let ebpf_config = ebpf_config(&config.ebpf);
		let max_buffered_events = ebpf_config.max_buffered_events;

		ebpf::configure_pids(&mut self.ebpf, ebpf_config.pids)?;
		ebpf::load_and_attach_bpf(&mut self.ebpf)?;

		let running = Arc::clone(&self.running);
		let mut handles = Vec::new();

		let mut perf_array = PerfEventArray::try_from(
			self.ebpf.take_map(EVENT_MAP).expect("Failed to take EVENTS map"),
		)?;
		// TODO: refactor this into a new submodule such as ebpf_collector
		for cpu_id in online_cpus().expect("Get CPU id error") {
			let mut buf = perf_array.open(
				cpu_id,
				Some(optimal_page_count(size_of::<Message>(), max_buffered_events as usize)),
			)?;
			let output = self.output.clone();
			let run = Arc::clone(&running);

			let handle = spawn_blocking(move || {
				block_on(async {
					let mut buffers = (0..max_buffered_events)
						.map(|_| BytesMut::with_capacity(size_of::<Message>()))
						.collect::<Vec<_>>();
					while run.load(Ordering::Relaxed) {
						let events = match buf.read_events(&mut buffers) {
							Ok(events) => events,
							Err(_) => Events { read: 0, lost: 0 },
						};

						// checking out lost events
						if events.lost > 0 || events.read > 0 {
							// TODO: handle lost events
						}

						for buf in buffers.iter().take(events.read) {
							let message = Message::decode(buf);
							debug!("Received message {}", message.pid);
							// info!("Received message {:?}", json!(message));
							if let Err(e) = output.send(message) {
								error!("Error sending message: {:?}", e);
							}
						}
					}
					Ok(())
				})
			});
			handles.push(handle);
		}
		self.handles.replace(handles);
		Ok(())
	}

	async fn stop(&mut self) -> Result<Self::Output> {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("{} is already stopped.", self.name());
			return Ok(());
		}

		info!("Stopping {} module...", self.name());

		if let Some(threads) = self.handles.take() {
			for thread in threads {
				thread.abort();
			}
		}
		info!("{} stopped.", self.name());
		Ok(())
	}
}
