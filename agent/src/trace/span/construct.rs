use super::{Cache, CacheEntry, SessionKey, Span};
use crossbeam_channel::{Receiver, Sender, select};
use std::time::{Duration, SystemTime};
use trace_common::{message::MessageType, structs::Data};

pub async fn construct_spans(message_receiver: Receiver<Data>, span_sender: Sender<Span>) {
	let cache = Cache::new();
	let _interval = tokio::time::interval(Duration::from_secs(10));

	loop {
		let span_sender = span_sender.clone();
		// select! {
		// interval.tick() => _ => process_timeout(&cache),
		if let Ok(data) = message_receiver.recv() {
			input_data(span_sender, &cache, data);
		}
	}
}

fn input_data(span_sender: Sender<Span>, cache: &Cache, message: Data) {
	let key = SessionKey::new(message.quintuple, message.protocol, message.uuid);

	let mut entry = cache.entry(key).or_insert(CacheEntry::new());

	match message.type_ {
		MessageType::Request => entry.request_input(message),
		MessageType::Response => entry.response_input(message),
		MessageType::Unknown => {},
	}
	while let (Some((req_time, req)), Some((res_time, res))) =
		(entry.request_first(), entry.response_first())
	{
		if req_time < res_time {
			entry.request_output();
			entry.response_output();
			let span = Span::new(req, res);
			span_sender.send(span).expect("Failed to send span");
		} else {
			entry.response_output();
		}
	}
}

fn process_timeout(cache: &Cache) {
	let now = SystemTime::now();
	cache.retain(|_, entry| {
		now.duration_since(entry.last_accessed()).unwrap() <= Duration::from_secs(10)
	});
}
