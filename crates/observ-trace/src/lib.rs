use arc_swap::access::Access;
use aya::{
	Ebpf,
	maps::{AsyncPerfEventArray, perf::Events},
	util::online_cpus,
};
use bytes::BytesMut;
use ebpf_manager::utils::{optimal_page_count, unlock_memory};
pub use error::TraceError;
use log::{info, warn};
use observ_config::{TraceAccess, TraceConfig, ebpf_config, trace_config};
use observ_core::Module;
use observ_event::span::Span;
use observ_runtime::handle;
use observ_trace_common::{Message, maps::EVENT_MAP};
use span::SpanConstructor;
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use tokio::{
	sync::mpsc::{self, Sender},
	task::JoinHandle,
	time,
};

mod ebpf;
mod error;
mod span;

type Result<T> = std::result::Result<T, TraceError>;

pub struct TraceCollector {
	config: TraceAccess,
	ebpf: Ebpf,
	output: Sender<Span>,
	span_constructor: Option<SpanConstructor>,
	running: Arc<AtomicBool>,
	handles: Option<Vec<JoinHandle<Result<()>>>>,
}

impl TraceCollector {
	pub fn new(output: Sender<Span>) -> Result<Self> {
		info!("Initializing Trace Collector");
		let config = trace_config();
		unlock_memory();
		let ebpf = ebpf::prepare_ebpf()?;
		Ok(Self {
			config,
			ebpf,
			output,
			span_constructor: None,
			running: Arc::new(AtomicBool::new(false)),
			handles: None,
		})
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
		let (message_sender, message_receiver) = mpsc::channel(1024);
		let span_sender = self.output.clone();
		let mut span_constructor = SpanConstructor::new(message_receiver, span_sender);
		span_constructor.start()?;

		let config = self.config.load();
		let ebpf_config = ebpf_config(&config.name);
		let max_buffered_events = ebpf_config.max_buffered_events;

		ebpf::configure_pids(&mut self.ebpf, ebpf_config.pids)?;
		ebpf::load_and_attach_bpf(&mut self.ebpf)?;

		let running = Arc::clone(&self.running);
		let mut handles = Vec::new();

		let mut perf_array = AsyncPerfEventArray::try_from(
			self.ebpf.take_map(EVENT_MAP).expect("Failed to take EVENTS map"),
		)?;
		// TODO: refactor this into a new submodule such as ebpf_collector
		for cpu_id in online_cpus().expect("Get CPU id error") {
			let mut buf = perf_array.open(
				cpu_id,
				Some(optimal_page_count(size_of::<Message>(), max_buffered_events as usize)),
			)?;
			let message_sender = message_sender.clone();
			let run = Arc::clone(&running);

			let handle = handle().spawn(async move {
				let mut buffers = (0..max_buffered_events)
					.map(|_| BytesMut::with_capacity(size_of::<Message>()))
					.collect::<Vec<_>>();
				let timeout = time::Duration::from_millis(10);
				while run.load(Ordering::Relaxed) {
					let events = match time::timeout(timeout, buf.read_events(&mut buffers)).await {
						Ok(events) => events?,
						Err(_) => Events { read: 0, lost: 0 },
					};

					// checking out lost events
					if events.lost > 0 || events.read > 0 {
						// TODO: handle lost events
					}

					for buf in buffers.iter().take(events.read) {
						let message = Message::decode(buf);
						// info!("Received message {}", message.pid);
						// info!("Received message {:?}", json!(message));
						message_sender.send(message).await.expect("Error sending message");
					}
				}
				Ok(())
			});
			handles.push(handle);
		}
		self.span_constructor.replace(span_constructor);
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
		if let Some(mut constructor) = self.span_constructor.take() {
			constructor.stop().await?;
		}
		info!("{} stopped.", self.name());
		Ok(())
	}
}
