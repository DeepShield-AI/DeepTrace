use serde::Serialize;
use trace_common::structs::Data;

mod construct;
mod output;

pub use construct::construct_spans;
pub use output::spans_output;

#[derive(Serialize)]
pub struct Span {
	req: Data,
	resp: Data,
}

impl Span {
	pub fn new(req: Data, resp: Data) -> Self {
		Self { req, resp }
	}
}