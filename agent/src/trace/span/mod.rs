use cache::Cache;
pub use error::Error as SpanError;
pub use module::SpanConstructor;
use serde::Serialize;
use std::ffi::CStr;
use trace_common::{
	protocols::L7Protocol,
	structs::{Data, Direction, Payload, Syscall},
};

mod cache;
mod error;
mod module;
mod spantag;

pub use spantag::SpanTag;

#[derive(Serialize)]
pub struct Span {
	tag: SpanTag,
	metric: SpanMetric,
	content: SpanContent,
}

#[derive(Serialize)]
pub struct SpanMetric {
	start_time: u64,
	end_time: u64,
	duration: u64,
	req_size: usize,
	resp_size: usize,
}

#[derive(Serialize)]
pub struct SpanContent {
	#[serde(serialize_with = "serialize_payload")]
	req_content: Payload,
	#[serde(serialize_with = "serialize_payload")]
	resp_content: Payload,
}
impl Span {
	pub async fn new(req: Data, resp: Data) -> Self {
		let tag = SpanTag::set_tags(&req, &resp).await;

		let metric = SpanMetric {
			start_time: req.timestamp_ns,
			end_time: resp.timestamp_ns,
			duration: resp.timestamp_ns - req.timestamp_ns,
			req_size: req.payload.len as usize,
			resp_size: resp.payload.len as usize,
		};

		let content = SpanContent { req_content: req.payload, resp_content: resp.payload };

		Self { tag, metric, content }
	}
}

fn serialize_payload<S>(payload: &Payload, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	let s = String::from_utf8_lossy(&payload.buf[..payload.len as usize]);
	serializer.serialize_str(&s)
}
