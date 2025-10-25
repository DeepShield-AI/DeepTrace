use serde::Serialize;

#[derive(Serialize)]
pub(in crate::span) struct SpanMetric {
	pub start_time: u64,
	pub end_time: u64,
	pub duration: u64,
	pub req_size: usize,
	pub resp_size: usize,
}
