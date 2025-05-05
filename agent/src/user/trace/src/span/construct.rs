use std::time::{Duration, SystemTime};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use trace_common::{message::MessageType, structs::Data};

use crate::{CacheEntry, Cache, SessionKey};

use super::Span;
pub async fn construct_spans(
	cache: Cache,
	mut message_receiver: UnboundedReceiver<Data>,
	span_sender: UnboundedSender<String>,
) {
	let mut interval = tokio::time::interval(Duration::from_secs(10));

	loop {
		let sender = span_sender.clone();
		tokio::select! {
			_ = interval.tick() => process_timeout(&cache),
			data = message_receiver.recv() => {
				if let Some(data) = data {
					input_data(sender, &cache, data);
				}
			}
		}
	}
}

fn input_data(span_sender: UnboundedSender<String>, cache: &Cache, message: Data) {
	let key = SessionKey::new(message.quintuple, message.protocol, message.uuid);

	let mut entry = cache.entry(key).or_insert(CacheEntry::new());

	match message.type_ {
		MessageType::Request => entry.request_input(message),
		MessageType::Response => entry.response_input(message),
		MessageType::Unknown => unreachable!(),
	}
	while let (Some((req_time, req)), Some((res_time, res))) =
		(entry.request_first(), entry.response_first())
	{
		if req_time < res_time {
			entry.request_output();
			entry.response_output();
			let span = Span::new(req, res);
			let span = serde_json::to_string_pretty(&span).unwrap();
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