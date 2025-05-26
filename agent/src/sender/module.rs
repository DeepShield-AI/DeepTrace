use super::{SendError, Sendable, TransportStrategy};
use crate::{
	app::runtime::{block_on, spawn_blocking},
	config::{SenderAccess, sender_config},
};
use arc_swap::access::Access;
use crossbeam_channel::{Receiver, RecvTimeoutError};
use log::{info, warn};
use std::{
	marker::PhantomData,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::{Duration, Instant},
};
use tokio::task::JoinHandle;

pub struct Sender<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
{
	backend: T,
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
	pub async fn process(&mut self) -> Result<(), SendError> {
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
		for item in batch.drain(..) {
			self.backend.send(item).await?;
		}
		self.backend.flush().await?;
		Ok(())
	}
}

pub(crate) struct SenderProcess<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
	SendError: From<<T as TransportStrategy<S>>::Error>,
{
	running: Arc<AtomicBool>,
	name: &'static str,
	receiver: Receiver<S>,
	thread: Option<JoinHandle<Result<(), SendError>>>,
	_marker: PhantomData<T>,
}

impl<S, T> SenderProcess<S, T>
where
	S: Sendable,
	T: TransportStrategy<S>,
	SendError: From<<T as TransportStrategy<S>>::Error>,
{
	pub fn new(name: &'static str, input: Receiver<S>) -> Self {
		Self {
			running: Default::default(),
			name,
			receiver: input,
			thread: None,
			_marker: PhantomData,
		}
	}
	fn name(&self) -> &str {
		self.name
	}

	pub fn start(&mut self, backend: T) -> Result<(), SendError> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("{} sender is already running.", self.name);
			return Ok(());
		}
		let mut sender = Sender {
			backend,
			receiver: self.receiver.clone(),
			running: self.running.clone(),
			config: sender_config(),
		};
		self.thread = Some(spawn_blocking(move || block_on(async { sender.process().await })));
		// self.thread = Some(spawn(async move { sender.process().await }));
		// thread::Builder::new()
		// 	.name(format!("{}-sender", self.name))
		// 	.spawn(|| block_on(async move { sender.spawn().await }))
		// 	.expect("Failed to spawn sender thread"),
		// );
		info!("{} sender started.", &self.name);
		Ok(())
	}

	pub async fn stop(&mut self) -> Result<(), SendError> {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("{} sender is not running.", self.name);
			return Ok(());
		}
		if let Some(thread) = self.thread.take() {
			thread.await.expect("Failed to join sender thread")?;
		}
		Ok(())
	}
}
