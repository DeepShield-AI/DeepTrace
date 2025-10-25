use super::{CodecEncodeError, Encoder};
pub use builder::JsonEncoderBuilder;
use bytes::{BufMut, BytesMut};
use observ_core::Sendable;
use serde::Serialize;

mod builder;

pub struct JsonEncoder {
	pretty: bool,
}

impl<S> Encoder<S> for JsonEncoder
where
	S: Sendable + Serialize,
{
	type Error = CodecEncodeError;

	fn encode(&mut self, records: S, out: &mut BytesMut) -> Result<(), Self::Error> {
		let writer = out.writer();
		if self.pretty {
			serde_json::to_writer_pretty(writer, &records)?;
		} else {
			serde_json::to_writer(writer, &records)?;
		}
		Ok(())
	}
}
