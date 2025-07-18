use super::{Cache, Span, SpanError};
use crate::{
	Module,
	app::runtime::{block_on, spawn_blocking},
	config::{SpanAccess, span_config},
};
use arc_swap::access::Access;
use crossbeam_channel::{Receiver, RecvError, Sender};
use log::{info, warn};
use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};
use tokio::{task::JoinHandle, time::Instant};
use trace_common::structs::Data;

pub struct SpanConstructor {
	running: Arc<AtomicBool>,
	input: Receiver<Data>,
	output: Sender<Span>,
	config: SpanAccess,
	handle: Option<JoinHandle<()>>,
}

impl SpanConstructor {
	pub fn new(input: Receiver<Data>, output: Sender<Span>) -> Self {
		Self { running: Default::default(), input, output, config: span_config(), handle: None }
	}
}

impl Module for SpanConstructor {
	type Error = SpanError;
	fn name(&self) -> &str {
		"SpanConstructor"
	}

	fn start(&mut self) -> Result<(), Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			return Ok(());
		}
		info!("Span constructor started");

		let input = self.input.clone();
		let output = self.output.clone();
		let running = self.running.clone();

		self.handle =
			Some(spawn_blocking(move || block_on(async { construct_spans(input, output,running).await })));
		Ok(())
	}

	async fn stop(&mut self) -> Result<(), Self::Error> {
		if !self.running.swap(false, Ordering::SeqCst) {
			return Ok(());
		}
		if let Some(handle) = self.handle.take() {
			handle.await.expect("Failed to join span constructor thread");
		}
		println!("Span constructor stopped");
		Ok(())
	}
}
async fn construct_spans(message_receiver: Receiver<Data>, span_sender: Sender<Span>,running:Arc<AtomicBool>) {
	let cache = Cache::new();
	let cleanup_interval = Duration::from_secs(20);
	let mut last_cleanup = Instant::now();
	while running.load(Ordering::Relaxed) {
		// info!("Span constructor is running");
	

		if let Ok(data) = message_receiver.try_recv() {
			let span_sender = span_sender.clone();
			cache.process(data, span_sender).await;
		}
		if last_cleanup.elapsed() >= cleanup_interval {
			cache.cleanup_expired();
			last_cleanup = Instant::now();
		}

	}
}
