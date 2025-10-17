use bytes::BytesMut;
pub use error::CodecEncodeError;

pub mod csv;
mod error;
pub mod json;

pub trait Encoder<S>: Send + 'static {
	type Error;
	fn encode(&mut self, item: S, buffer: &mut BytesMut) -> Result<(), Self::Error>;
}
