use super::{App, Module, context::init, runtime::spawn, state, terminate};
use crate::{
	AgentError,
	provenance::Provenance,
	sender::{Elastic, FlatFile, SenderProcess},
	synchronizer::Synchronizer,
	trace::{SpanConstructor, TraceModule},
};
use log::info;
use std::sync::atomic::Ordering;

impl App {
	pub fn new(config: impl AsRef<str>) -> Result<Self, AgentError> {
		// Add log initialization here
		env_logger::builder().init();
		// console_subscriber::init();
		
		init(config)?;
		Ok(Self { handle: None })
	}

	pub fn start(&mut self) {
		self.handle = Some(spawn(run()));
	}

	pub fn stop(&mut self) {
		terminate();
		self.handle.take().expect("Failed to stop app").abort();
		info!("App stopped");
	}
}

async fn run() -> Result<(), AgentError> {
	let (message_sender, message_receiver) = crossbeam_channel::unbounded();
	let (span_sender, span_receiver) = crossbeam_channel::unbounded();

	let mut synchronizer = Synchronizer::new();
	synchronizer.start()?;

	// let mut ebpf_log = SenderProcess::new("ebpf", message_receiver.clone());
	// ebpf_log.start(FlatFile::new("message.txt").await.expect("Flat file error"))?;
	let mut span_log = SenderProcess::new("span", span_receiver);
	// span_log.start(FlatFile::new("spans.txt").await.expect("Flat file error"))?;
	span_log.start(Elastic::new().await.expect("Elastic error"))?;

	let mut span_constructor = SpanConstructor::new(message_receiver, span_sender);
	span_constructor.start()?;
	let mut trace = TraceModule::new(message_sender).expect("Failed to create eBPF module");
	trace.start()?;

	let mut provenance = Provenance::new()?;
	provenance.start()?;

	// let mut components: Vec<Box<&dyn Module<Error = dyn Into<AgentError>>>> = Vec::new();
	// components.push(Box::new(&trace));
	// components.push(Box::new(&config));
	loop {
		if state().load(Ordering::Relaxed) {
			// for component in &mut components {
			// 	component.stop().await?;
			// }
			synchronizer.stop().await?;
			provenance.stop().await?;
			trace.stop().await?;
			span_constructor.stop().await?;
			span_log.stop().await?;
			// ebpf_log.stop().await?;
			return Ok(());
		}
	}
}
