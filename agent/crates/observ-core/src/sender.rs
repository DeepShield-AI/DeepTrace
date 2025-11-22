use bytes::BytesMut;
use std::future::Future;

/// A abstraction for sending data and serialize data.
pub trait Sendable: Send + 'static {
	// Encode data to bytes stream and wait for sender to send
	fn encode(&self, _: &mut BytesMut) -> Result<(), std::io::Error> {
		Ok(())
	}
}

/// A abstraction for sending data.
pub trait Sender<S: Sendable>: Send + 'static {
	type Error;
	/// Sends a message. Cache the message to encoder buffer.
	fn send(&mut self, message: BytesMut) -> impl Future<Output = Result<(), Self::Error>> + Send;
	/// Flushes the encoder buffer and sends the data.
	/// This is usually called when the buffer is full or when the transport is stopped.
	fn flush(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
