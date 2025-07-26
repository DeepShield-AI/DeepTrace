use super::{App, Module, context::init, runtime::spawn, state, terminate};
use crate::{
	AgentError,
	metric::MetricCollector,
	// provenance::Provenance,
	sender::{Elastic, FlatFile, SenderProcess},
	synchronizer::Synchronizer,
	trace::{SpanConstructor, TraceModule},
};
use crossbeam_channel::unbounded;
use log::info;
use rocket::yansi::Paint;
use std::{sync::atomic::Ordering, time::Duration};
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

		info!("Starting agent");
		println!("{}", Paint::green("Agent started"));
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
	println!("{}", Paint::green("Agent run started"));
	info!("Starting agent run");
	let (message_sender, message_receiver) = crossbeam_channel::unbounded();
	let (span_sender, span_receiver) = crossbeam_channel::unbounded();

	//let mut synchronizer = Synchronizer::new();
	//synchronizer.start()?;
	println!("{}", Paint::green("Agent spanlog start"));
	// let mut ebpf_log = SenderProcess::new("ebpf", message_receiver.clone());
	// ebpf_log.start(FlatFile::new("message.txt").await.expect("Flat file error"))?;
	let mut metric_module = MetricCollector::new().expect("Failed to create metric module");
	metric_module.start()?; // 启动采集器
	let mut span_log = SenderProcess::new("span", span_receiver);
	span_log.start(FlatFile::new("spans.txt").await.expect("Flat file error"))?;
	//span_log.start(Elastic::new().await.expect("Elastic error"))?;
	println!("{}", Paint::green("Agent spanconstructor start"));
	let mut span_constructor = SpanConstructor::new(message_receiver, span_sender);
	span_constructor.start()?;
	println!("{}", Paint::green("Agent spanTrace start"));
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
			//synchronizer.stop().await?;
			// provenance.stop().await?;
			println!("{}", Paint::red("Agent trace catch"));
			trace.stop().await?; //停止tracemodule时程序异常结束，没有执行后续代码
			println!("{}", Paint::red("Agent trace catch"));
			span_constructor.stop().await?;
			println!("{}", Paint::red("Agent span constructor catch"));
			span_log.stop().await?;
			println!("{}", Paint::red("Agent spanlog catch"));
			metric_module.stop().await?;
			// ebpf_log.stop().await?;
			return Ok(());
		}
	}
}
