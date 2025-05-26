use super::Span;
use crossbeam_channel::Sender;
use dashmap::DashMap;
use std::time::SystemTime;
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
}
impl SessionKey {
	pub fn new(quintuple: Quintuple, protocol: L7Protocol, uuid: u32) -> Self {
		Self { quintuple, protocol, uuid }
	}
}
#[derive(Debug)]
pub struct CacheEntry {
	request: Option<Data>,
	response: Option<Data>,
	last_accessed: SystemTime,
}

impl CacheEntry {
	pub fn new() -> Self {
		Self { request: None, response: None, last_accessed: SystemTime::now() }
	}

	fn try_match(&mut self, sender: &crossbeam_channel::Sender<Span>) {
		if let (Some(req), Some(res)) = (self.request, self.response) {
			if req.timestamp_ns <= res.timestamp_ns {
				let span = Span::new(req, res);
				sender.send(span).expect("Failed to send span");
				self.request.take();
				self.response.take();
			}
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

	pub fn process(&self, data: Data, sender: Sender<Span>) {
		let key = SessionKey::new(data.quintuple, data.protocol, data.uuid);
		let mut entry = self.inner.entry(key).or_insert(CacheEntry::new());
		entry.last_accessed = SystemTime::now();
		match data.type_ {
			MessageType::Request => entry.request.replace(data),
			MessageType::Response => entry.response.replace(data),
			MessageType::Unknown => None,
		};

		entry.try_match(&sender);
	}

	pub fn cleanup_expired(&self) {
		let now = SystemTime::now();
		self.inner.retain(|_, entry| {
			now.duration_since(entry.last_accessed)
				.map(|d| d < Self::EXPIRATION_DURATION)
				.unwrap_or(false)
		});
	}
}
