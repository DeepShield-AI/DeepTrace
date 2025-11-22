use bytes::BytesMut;
use codec::encode::Encoder;
use crossbeam_channel::{Receiver, RecvTimeoutError};
pub use error::SendError;
use log::{info, warn, error};
use observ_core::{Module, Sendable};
use observ_runtime::{block_on, spawn_blocking};
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use tokio::task::JoinHandle;
pub mod elastic;
mod error;
pub mod file;

pub struct Sender<T, S, E>
where
	T: Sendable,
	S: observ_core::Sender<T>,
	E: Encoder<T>,
{
	name: &'static str,
	receiver: Receiver<T>,
	encoder: Option<E>,
	sender: Option<S>,
	running: Arc<AtomicBool>,
	handle: Option<JoinHandle<Result<(), SendError>>>,
}

impl<T, S, E> Sender<T, S, E>
where
	T: Sendable,
	S: observ_core::Sender<T>,
	E: Encoder<T>,
{
	pub fn new(name: &'static str, receiver: Receiver<T>, sender: S, encoder: E) -> Self {
		Self {
			name,
			running: Arc::new(AtomicBool::new(false)),
			receiver,
			sender: Some(sender),
			encoder: Some(encoder),
			handle: None,
		}
	}
}

impl<T, S, E> Module for Sender<T, S, E>
where
	T: Sendable,
	S: observ_core::Sender<T>,
	E: Encoder<T>,
	SendError: From<<E as Encoder<T>>::Error> + From<<S as observ_core::Sender<T>>::Error>,
	<E as Encoder<T>>::Error: std::fmt::Debug,
	<S as observ_core::Sender<T>>::Error: std::fmt::Debug,
{
	type Config = ();
	type Error = SendError;
	type Output = ();

	fn name(&self) -> &str {
		self.name
	}

	fn start(&mut self) -> Result<Self::Output, Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("{} sender is already running.", self.name);
			return Ok(());
		}
		let name = self.name.to_string();
		let running = Arc::clone(&self.running);
		let receiver = self.receiver.clone();
		let mut sender = self.sender.take().unwrap();
		let mut encoder = self.encoder.take().unwrap();
		self.handle = Some(spawn_blocking(move || {
			block_on(async {
				while running.load(Ordering::Relaxed) {
					match receiver.recv_timeout(std::time::Duration::from_secs(1)) {
						Ok(message) => {
							// debug!("Sending message");
							let mut encoded = BytesMut::new();
							if let Err(e) = encoder.encode(message, &mut encoded) {
								error!("Failed to encode message: {:?}", e);
								continue;
							}
							// debug!("Encoded message: {encoded:?}");
							if let Err(e) = sender.send(encoded).await {
								error!("Failed to send message: {:?}", e);
							}
						},
						Err(RecvTimeoutError::Timeout) =>
							if let Err(e) = sender.flush().await {
								error!("Failed to flush sender: {:?}", e);
							},
						Err(RecvTimeoutError::Disconnected) => {
							warn!("Sender receiver disconnected.");
							break;
						},
					}
				}
				// sender.flush().await?;
				info!("{} sender stopped.", name);
				Ok(())
			})
		}));
		info!("{} sender started.", self.name);
		Ok(())
	}

	async fn stop(&mut self) -> Result<Self::Output, Self::Error> {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("{} sender is already stopped.", self.name);
			return Ok(());
		}

		if let Some(thread) = self.handle.take() {
			thread.abort();
			// .unwrap_or_else(|_| panic!("Failed to join {} sender thread", self.name))?;
		}
		info!("{} sender stopped.", self.name);
		Ok(())
	}
}
