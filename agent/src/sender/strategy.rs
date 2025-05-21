use super::SendError;
use std::future::Future;

pub trait TransportStrategy<T: Sendable>: Send + Sync + 'static {
	type Error: Into<SendError>;
	fn send(&mut self, item: T) -> impl Future<Output = Result<(), Self::Error>> + Send;
	fn flush(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
pub trait Sendable: Send + Sync + 'static {}

impl<T: Send + Sync + 'static> Sendable for T {}
