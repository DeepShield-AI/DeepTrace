use super::JsonEncoder;

#[derive(Default)]
pub struct JsonEncoderBuilder {
	pretty: bool,
}

impl JsonEncoderBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub const fn pretty(mut self, pretty: bool) -> Self {
		self.pretty = pretty;
		self
	}

	pub const fn build(self) -> JsonEncoder {
		JsonEncoder { pretty: self.pretty }
	}
}
