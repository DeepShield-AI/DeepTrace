use super::{
	App, Module,
	context::init,
	runtime::spawn,
	state,
	statistic::{LogStore, Statistic, add_log, init_statistic},
	terminate,
};
use crate::{
	AgentError,
	config::agent_config,
	metric::MetricCollector,
	sender::{Elastic, FlatFile, SenderProcess},
	synchronizer::Synchronizer,
	trace::{SpanConstructor, TraceModule},
};
use arc_swap::access::Access;
use log::info;
use sha2::{Digest, Sha256};
use std::{sync::atomic::Ordering, time::Duration};
use tokio::time::sleep;

impl App {
	pub async fn new(config: impl AsRef<str>) -> Result<Self, AgentError> {
		// Add log initialization here
		env_logger::builder().init();
		// console_subscriber::init();

		init(config)?;
		let name = agent_config().load().name.clone();
		let mut hasher = Sha256::new();
		hasher.update(name.as_bytes());
		let lcuuid = format!("{:x}", hasher.finalize());
		let initial_stat = Statistic {
			cpu_usage: 0.0,
			memory_usage: 0.0,
			name: name.clone(),
			lcuuid: lcuuid.clone(),
			span_num: 0,
			timestamp: chrono::Utc::now().to_rfc3339(),
			log_store: LogStore { agent_name: name.clone(), lcuuid, logs: Vec::new() },
		};
		init_statistic(initial_stat);
		add_log("info", &format!("App initialized with name: {}", name)).await;
		Ok(Self { handle: None })
	}

	pub fn start(&mut self) {
		self.handle = Some(spawn(run()));
		info!("Starting agent");
	}

	pub async fn stop(&mut self) {
		terminate();

		if let Some(handle) = self.handle.take() {
			if !handle.is_finished() {
				info!("Waiting for task to finish...");
				if let Err(e) = handle.await {
					info!("Task failed or was aborted: {:?}", e);
				}
			}
		} else {
			info!("No handle found, task may have already been stopped");
		}

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
	let mut metric = MetricCollector::new().expect("Failed to create metric module");
	metric.start()?;
	let mut span_log = SenderProcess::new("span", span_receiver);
	span_log.start(FlatFile::new("spans.txt").await.expect("Flat file error"))?;
	// span_log.start(Elastic::new().await.expect("Elastic error"))?;

	let mut span_constructor = SpanConstructor::new(message_receiver, span_sender);
	span_constructor.start()?;
	let mut trace = TraceModule::new(message_sender).expect("Failed to create eBPF module");
	trace.start()?;

	// let mut provenance = Provenance::new()?;
	// provenance.start()?;

	// let mut components: Vec<Box<&dyn Module<Error = dyn Into<AgentError>>>> = Vec::new();
	// components.push(Box::new(&trace));
	// components.push(Box::new(&config));
	loop {
		// info!("App is running");
		if state().load(Ordering::Relaxed) {
			// for component in &mut components {
			// 	component.stop().await?;
			// }
			synchronizer.stop().await?;
			// provenance.stop().await?;
			trace.stop().await?;
			span_constructor.stop().await?;
			span_log.stop().await?;
			metric.stop().await?;
			// ebpf_log.stop().await?;
			return Ok(());
		}
	}
}
