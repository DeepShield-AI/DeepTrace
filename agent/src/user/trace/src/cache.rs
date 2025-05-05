use dashmap::{DashMap, Entry};
use std::{
	collections::VecDeque,
	ops::Add,
	sync::Arc,
	time::{SystemTime, UNIX_EPOCH},
};
use tokio::time::Duration;
use trace_common::{
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
	req_queue: VecDeque<(SystemTime, Data)>,
	res_queue: VecDeque<(SystemTime, Data)>,
	last_accessed: SystemTime,
}

impl CacheEntry {
	pub fn new() -> Self {
		Self {
			req_queue: VecDeque::with_capacity(16),
			res_queue: VecDeque::with_capacity(16),
			last_accessed: SystemTime::now(),
		}
	}
	pub fn request_input(&mut self, message: Data) {
		let time = UNIX_EPOCH.add(Duration::from_nanos(message.timestamp_ns));
		self.req_queue.push_back((time, message));
	}
	pub fn response_input(&mut self, message: Data) {
		let time = UNIX_EPOCH.add(Duration::from_nanos(message.timestamp_ns));
		self.res_queue.push_back((time, message));
	}
	pub fn request_first(&mut self) -> Option<(SystemTime, Data)> {
		self.req_queue.front().cloned()
	}
	pub fn response_first(&mut self) -> Option<(SystemTime, Data)> {
		self.res_queue.front().cloned()
	}
	pub fn request_output(&mut self) -> Option<(SystemTime, Data)> {
		self.req_queue.pop_front()
	}
	pub fn response_output(&mut self) -> Option<(SystemTime, Data)> {
		self.res_queue.pop_front()
	}
	pub fn last_accessed(&self) -> SystemTime {
		self.last_accessed
	}
}

pub struct Cache {
	inner: Arc<DashMap<SessionKey, CacheEntry>>,
}

impl Cache {
	pub fn new() -> Self {
		Self { inner: Arc::new(DashMap::with_capacity(102400)) }
	}

	pub fn entry(&self, key: SessionKey) -> Entry<SessionKey, CacheEntry> {
		self.inner.entry(key)
	}
	pub fn retain<F>(&self, f: F)
	where
		F: FnMut(&SessionKey, &mut CacheEntry) -> bool,
	{
		self.inner.retain(f)
	}
}
