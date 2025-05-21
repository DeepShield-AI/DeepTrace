use super::{
	App, Module,
	context::{self, init, state},
	runtime::spawn,
};
use crate::{
	AgentError,
	config::ConfigModule,
	sender::{Elastic, FlatFile, SenderProcess},
	trace::{SpanConstructor, TraceModule},
};
use log::info;
use std::sync::atomic::Ordering;

impl App {
	pub fn new(config: impl AsRef<str>) -> Result<Self, AgentError> {
		// Add log initialization here
		env_logger::builder().init();
		init(config)?;
		Ok(Self { handle: None })
	}

	pub fn start(&mut self) {
		self.handle = Some(spawn(run()));
	}

	pub fn stop(&mut self) {
		context::terminate();
		self.handle.take().expect("Failed to stop app").abort();
		info!("App stopped");
	}
}

async fn run() -> Result<(), AgentError> {
	let (message_sender, message_receiver) = crossbeam_channel::unbounded();
	let (span_sender, span_receiver) = crossbeam_channel::unbounded();
	let mut ebpf_log = SenderProcess::new(
		"ebpf",
		FlatFile::new("message.txt").await.expect("Flat file error"),
		message_receiver.clone(),
	);
	ebpf_log.start()?;
	let mut span_log =
		SenderProcess::new("span", Elastic::new().await.expect("Elastic error"), span_receiver);
	// let mut span_log = SenderProcess::new(
	// 	"span",
	// 	FlatFile::new("span.txt").await.expect("Flat file error"),
	// 	span_receiver,
	// );
	span_log.start()?;
	let mut span_constructor = SpanConstructor::new(message_receiver, span_sender);
	span_constructor.start()?;
	let mut trace = TraceModule::new(message_sender).expect("Failed to create eBPF module");
	trace.start()?;

	let mut config = ConfigModule::new();
	config.start()?;

	// let mut component: Vec<Box<dyn Module>> = vec![];
	// component.push(Box::new(trace));
	// component.push(Box::new(config));
	loop {
		if state().load(Ordering::Relaxed) {
			// for component in &mut component {
			// 	component.stop().await?;
			// }
			config.stop()?;
			trace.stop()?;
			span_constructor.stop()?;
			ebpf_log.stop()?;
			span_log.stop()?;
			return Ok(());
		}
	}
}
