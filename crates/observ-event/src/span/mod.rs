use content::SpanContent;
use metric::SpanMetric;
use observ_trace_common::message::Message;
use serde::Serialize;

mod content;
mod metric;
mod tag;
#[derive(Serialize)]
pub struct Span {
	// tag: SpanTag,
	metric: SpanMetric,
	content: SpanContent,
}

impl Span {
	pub async fn new(req: Message, resp: Message) -> Self {
		// let tag = SpanTag::set_tags(&req, &resp).await;

		let metric = SpanMetric {
			start_time: req.timestamp_ns,
			end_time: resp.timestamp_ns,
			duration: resp.timestamp_ns - req.timestamp_ns,
			req_size: req.payload.len(),
			resp_size: resp.payload.len(),
		};

		let content = SpanContent { req_content: req.payload, resp_content: resp.payload };

		Self { metric, content }
		// Self { tag, metric, content }
	}
}
