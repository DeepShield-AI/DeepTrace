use observ_trace_common::{Buffer, MAX_PAYLOAD_SIZE, serialize_buffer};
use serde::Serialize;

#[derive(Serialize)]
pub(in crate::span) struct SpanContent {
	#[serde(serialize_with = "serialize_buffer")]
	pub req_content: Buffer<MAX_PAYLOAD_SIZE>,
	#[serde(serialize_with = "serialize_buffer")]
	pub resp_content: Buffer<MAX_PAYLOAD_SIZE>,
}
