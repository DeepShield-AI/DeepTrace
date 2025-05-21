use super::{SendError, Sendable, TransportStrategy};
use crate::{
	Module,
	app::runtime::block_on,
	config::{SenderAccess, sender_config},
};
use arc_swap::access::Access;
use crossbeam_channel::{Receiver, RecvTimeoutError};
use log::{info, warn};
use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	thread::{self, JoinHandle},
	time::{Duration, Instant},
};
use tokio::sync::Mutex;

pub struct Sender<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
{
	backend: Arc<Mutex<T>>,
	receiver: Receiver<S>,
	running: Arc<AtomicBool>,
	config: SenderAccess,
}

impl<S, T> Sender<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
	SendError: From<<T as TransportStrategy<S>>::Error>,
{
	const RECV_TIMEOUT: Duration = Duration::from_secs(3);
	pub async fn spawn(&mut self) -> Result<(), SendError> {
		let batch_size = self.config.load().batch_size;
		let mut batch = Vec::with_capacity(batch_size);

		while self.running.load(Ordering::Relaxed) {
			match self.receiver.recv_deadline(Instant::now() + Self::RECV_TIMEOUT) {
				Ok(item) => {
					batch.push(item);
					if batch.len() >= batch_size {
						self.flush(&mut batch).await?;
					}
				},
				Err(RecvTimeoutError::Timeout) => {
					if !batch.is_empty() {
						self.flush(&mut batch).await?;
					}
					tokio::time::sleep(Duration::from_secs(1)).await;
				},
				Err(RecvTimeoutError::Disconnected) => {
					warn!("Sender receiver disconnected.");
					break;
				},
			}
		}

		if !batch.is_empty() {
			self.flush(&mut batch).await?;
		}
		Ok(())
	}

	async fn flush(&mut self, batch: &mut Vec<S>) -> Result<(), SendError> {
		let mut backend = self.backend.lock().await;
		for item in batch.drain(..) {
			backend.send(item).await?;
		}
		backend.flush().await?;
		Ok(())
	}
}
pub struct SenderProcess<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
{
	running: Arc<AtomicBool>,
	name: &'static str,
	backend: Arc<Mutex<T>>,
	receiver: Receiver<S>,
	config: SenderAccess,
	thread: Option<JoinHandle<Result<(), SendError>>>,
}

impl<S, T> SenderProcess<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
	SendError: From<<T as TransportStrategy<S>>::Error>,
{
	pub fn new(name: &'static str, backend: T, input: Receiver<S>) -> Self {
		Self {
			running: Arc::new(AtomicBool::new(false)),
			name,
			backend: Arc::new(Mutex::new(backend)),
			receiver: input,
			config: sender_config(),
			thread: None,
		}
	}
}

impl<S, T> Module for SenderProcess<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
	SendError: From<<T as TransportStrategy<S>>::Error>,
{
	type Error = SendError;

	fn name(&self) -> &str {
		self.name
	}

	fn start(&mut self) -> Result<(), Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("{} sender is already running.", self.name);
			return Ok(());
		}
		let mut sender = Sender {
			backend: self.backend.clone(),
			receiver: self.receiver.clone(),
			running: self.running.clone(),
			config: self.config.clone(),
		};
		self.thread = Some(
			thread::Builder::new()
				.name(format!("{}-sender", self.name))
				.spawn(|| block_on(async move { sender.spawn().await }))
				.expect("Failed to spawn sender thread"),
		);
		info!("{} sender started.", &self.name);
		Ok(())
	}

	fn stop(&mut self) -> Result<(), Self::Error> {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("{} sender is not running.", self.name);
			return Ok(());
		}
		if let Some(thread) = self.thread.take() {
			thread.join().expect("Failed to join sender thread")?;
		}
		Ok(())
	}
}
