use super::Span;
use crossbeam_channel::Sender;
use dashmap::DashMap;
use log::info;
use lru::LruCache;
use serde_json::json;
use std::{num::NonZeroUsize, time::SystemTime};
use tokio::time::Duration;
use trace_common::{
	message::MessageType,
	protocols::L7Protocol,
	structs::{Data, Quintuple},
};

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct SessionKey {
	quintuple: Quintuple,
	protocol: L7Protocol,
	uuid: u32,
	pid: u32,
}
impl SessionKey {
	pub fn new(pid: u32, quintuple: Quintuple, protocol: L7Protocol, uuid: u32) -> Self {
		Self { pid, quintuple, protocol, uuid }
	}
}
#[derive(Debug)]
pub struct CacheEntry {
	messages: LruCache<u32, Data>,
	last_accessed: SystemTime,
}

impl CacheEntry {
	pub fn new() -> Self {
		Self {
			messages: LruCache::new(NonZeroUsize::new(1024).unwrap()),
			last_accessed: SystemTime::now(),
		}
	}
}

pub(super) struct Cache {
	inner: DashMap<SessionKey, CacheEntry>,
}

impl Cache {
	const EXPIRATION_DURATION: Duration = Duration::from_secs(10);
	pub fn new() -> Self {
		Self { inner: DashMap::with_capacity(10240) }
	}

	pub async fn process(&self, data: Data, sender: Sender<Span>) {
		let key = SessionKey::new(data.pid, data.quintuple, data.protocol, data.uuid);
		let mut entry = self.inner.entry(key).or_insert(CacheEntry::new());
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
						if prev.is_response() && prev.timestamp_ns > data.timestamp_ns =>
					{
						sender.send(Span::new(data, prev).await).expect("Failed to send span");
					},
					MessageType::Response
						if prev.is_request() && prev.timestamp_ns < data.timestamp_ns =>
					{
						sender.send(Span::new(prev, data).await).expect("Failed to send span");
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
						info!("Unexpected message: {:?}", json!(data));
						return;
					},
				};
			},
			None => {
				if entry.messages.len() >= 1024 {
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

	pub fn cleanup_expired(&self) {
		let now = SystemTime::now();
		info!("Cleaning up expired entries...");
		info!("Current cache size: {}", self.inner.len());
		let mut expired_count = 0;
		self.inner.retain(|_, entry| {
			match now
				.duration_since(entry.last_accessed)
				.map(|d| d < Self::EXPIRATION_DURATION)
				.unwrap_or(false)
			{
				true => true,
				false => {
					entry.messages.iter().for_each(|(_, msg)| {
						if msg.protocol == L7Protocol::Thrift {
							info!("Expired message: {:?}", json!(msg));
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
