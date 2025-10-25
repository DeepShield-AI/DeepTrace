use arc_swap::access::Access;
use dashmap::DashMap;
pub use error::SpanError;
use log::info;
use lru::LruCache;
use observ_config::{SpanAccess, span_config};
use observ_core::Module;
use observ_event::span::Span;
use observ_runtime::handle;
use observ_trace_common::{L7Protocol, Message, MessageType, Quintuple};
use std::{
	num::NonZeroUsize,
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::{Duration, SystemTime},
};
use tokio::{
	sync::mpsc::{Receiver, Sender},
	task::JoinHandle,
	time::Instant,
};

mod error;

#[derive(Eq, PartialEq, Hash)]
pub struct SessionKey {
	quintuple: Quintuple,
	protocol: L7Protocol,
	uuid: u32,
}
impl SessionKey {
	pub const fn new(quintuple: Quintuple, protocol: L7Protocol, uuid: u32) -> Self {
		Self { quintuple, protocol, uuid }
	}
}
#[derive(Debug)]
pub struct CacheEntry {
	messages: LruCache<u32, Message>,
	last_accessed: SystemTime,
}

impl CacheEntry {
	pub fn new(max_sockets: usize) -> Self {
		Self {
			messages: LruCache::new(NonZeroUsize::new(max_sockets).unwrap()),
			last_accessed: SystemTime::now(),
		}
	}
}

pub(super) struct Cache {
	inner: DashMap<SessionKey, CacheEntry>,
}

impl Cache {
	pub fn new() -> Self {
		Self { inner: DashMap::with_capacity(10240) }
	}

	fn cleanup_expired(&self, interval: Duration) {
		let now = SystemTime::now();
		info!("Cleaning up expired entries...");
		info!("Current cache size: {}", self.inner.len());
		let mut expired_count = 0;
		self.inner.retain(|_, entry| {
			match now.duration_since(entry.last_accessed).map(|d| d < interval).unwrap_or(false) {
				true => true,
				false => {
					entry.messages.iter().for_each(|(_, msg)| {
						if msg.protocol == L7Protocol::Thrift {
							// info!("Expired message: {:?}", json!(msg));
							expired_count += 1;
						}
					});

					false
				},
			}
		});
		info!("Expired entries: {}", expired_count);
		info!("Cache size after cleanup: {}", self.inner.len());
	}
}

// When config is updated, the span constructor don't need to be restarted, so
// we use Option to contain input and output
pub struct SpanConstructor {
	config: SpanAccess,
	input: Option<Receiver<Message>>,
	output: Option<Sender<Span>>,
	running: Arc<AtomicBool>,
	handle: Option<JoinHandle<()>>,
}

impl SpanConstructor {
	pub fn new(input: Receiver<Message>, output: Sender<Span>) -> Self {
		Self {
			config: span_config(),
			input: Some(input),
			output: Some(output),
			running: Default::default(),
			handle: None,
		}
	}
}

impl Module for SpanConstructor {
	type Config = ();
	type Output = ();
	type Error = SpanError;
	fn name(&self) -> &str {
		"Span Constructor"
	}

	fn start(&mut self) -> Result<(), Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			return Ok(());
		}
		info!("Span constructor started");

		let config = self.config.load();
		let mut input = self.input.take().ok_or(SpanError::MissingReceiver)?;
		let output = self.output.take().ok_or(SpanError::MissingSender)?;
		let running = Arc::clone(&self.running);

		self.handle = Some(handle().spawn(async move {
			let cache = Cache::new();
			let cleanup_interval = Duration::from_secs(config.cleanup_interval);
			let mut last_cleanup = Instant::now();
			while running.load(Ordering::Relaxed) {
				if let Some(data) = input.recv().await {
					let key = SessionKey::new(data.quintuple, data.protocol, data.uuid);
					let mut entry =
						cache.inner.entry(key).or_insert(CacheEntry::new(config.max_sockets));
					entry.last_accessed = SystemTime::now();

					let key = match data.type_ {
						MessageType::Request => data.seq + 1,
						MessageType::Response => data.seq,
						MessageType::Unknown => 0,
					};
					match entry.messages.pop(&key) {
						Some(prev) => {
							match data.type_ {
								MessageType::Request
									if prev.is_response() &&
										prev.timestamp_ns > data.timestamp_ns =>
								{
									output
										.send(Span::new(data, prev).await)
										.await
										.expect("Failed to send span");
								},
								MessageType::Response
									if prev.is_request() &&
										prev.timestamp_ns < data.timestamp_ns =>
								{
									output
										.send(Span::new(prev, data).await)
										.await
										.expect("Failed to send span");
								},
								_ if data.type_ != MessageType::Unknown => {
									match data.timestamp_ns > prev.timestamp_ns {
										true => {
											entry.messages.put(key, data);
										},
										false => {
											entry.messages.put(key, prev);
										},
									}
								},
								_ => {
									// info!("Unexpected message: {:?}", json!(data));
								},
							};
						},
						None => {
							if entry.messages.len() >= config.max_sockets {
								let flush_size = 100_u64;
								// Prevent too many logs from being cached
								for _ in 0..flush_size {
									let _ = entry.messages.pop_lru();
								}
							}
							entry.messages.put(key, data);
						},
					}
				}
				if last_cleanup.elapsed() >= cleanup_interval {
					cache.cleanup_expired(cleanup_interval);
					last_cleanup = Instant::now();
				}
			}
		}));
		Ok(())
	}

	async fn stop(&mut self) -> Result<(), Self::Error> {
		if !self.running.swap(false, Ordering::SeqCst) {
			return Ok(());
		}
		if let Some(handle) = self.handle.take() {
			handle.abort();
			// .expect("Failed to join span constructor thread");
		}
		info!("Span constructor stopped");
		Ok(())
	}
}
