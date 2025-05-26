use super::{Cache, Span, SpanError};
use crate::{
	Module,
	app::runtime::{block_on, spawn_blocking},
};
use crossbeam_channel::{Receiver, Sender};
use log::info;
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
	handle: Option<JoinHandle<()>>,
}

impl SpanConstructor {
	pub fn new(input: Receiver<Data>, output: Sender<Span>) -> Self {
		Self { running: Default::default(), input, output, handle: None }
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

		self.handle = Some(spawn_blocking(move || {
			block_on(async {
				construct_spans(input, output).await;
			})
		}));
		Ok(())
	}

	async fn stop(&mut self) -> Result<(), Self::Error> {
		if !self.running.swap(false, Ordering::SeqCst) {
			return Ok(());
		}
		if let Some(handle) = self.handle.take() {
			handle.await.expect("Failed to join span constructor thread");
		}
		Ok(())
	}
}

async fn construct_spans(message_receiver: Receiver<Data>, span_sender: Sender<Span>) {
	let cache = Cache::new();
	let cleanup_interval = Duration::from_secs(1);
	let mut last_cleanup = Instant::now();
	loop {
		let span_sender = span_sender.clone();

		if let Ok(data) = message_receiver.recv() {
			cache.process(data, span_sender);
		}
		if last_cleanup.elapsed() >= cleanup_interval {
			cache.cleanup_expired();
			last_cleanup = Instant::now();
		}
	}
}
