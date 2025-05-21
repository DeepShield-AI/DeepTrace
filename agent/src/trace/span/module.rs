use super::{Span, SpanError, construct_spans};
use crate::{Module, app::runtime::block_on};
use crossbeam_channel::{Receiver, Sender};
use log::info;
use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	thread::{self, JoinHandle},
};
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

		self.handle = Some(
			thread::Builder::new()
				.name("span-constructor".to_owned())
				.spawn(|| {
					block_on(async {
						construct_spans(input, output).await;
					})
				})
				.expect("Failed to spawn span constructor thread"),
		);
		Ok(())
	}

	fn stop(&mut self) -> Result<(), Self::Error> {
		if !self.running.swap(false, Ordering::SeqCst) {
			return Ok(());
		}
		if let Some(handle) = self.handle.take() {
			handle.join().expect("Failed to join span constructor thread");
		}
		Ok(())
	}
}
