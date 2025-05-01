use mercury_common::structs::Data;
use serde::Serialize;

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
