use crate::protocols::L7Protocol;
use serde::Serialize;

#[derive(Default, Clone, Copy, Serialize, Debug)]
pub enum MessageType {
	#[default]
	Unknown,
	Request,
	Response,
}

impl MessageType {
	pub fn debug(&self) -> &'static str {
		match self {
			MessageType::Unknown => "Unknown",
			MessageType::Request => "Request",
			MessageType::Response => "Response",
		}
	}
}

#[derive(Default)]
pub struct Message {
	pub uuid: u32,
	pub protocol: L7Protocol,
	pub type_: MessageType,
}

impl Message {
	pub fn new() -> Self {
		Default::default()
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for MessageType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MessageType::Unknown => f.write_str("Unknown"),
			MessageType::Request => f.write_str("Request"),
			MessageType::Response => f.write_str("Response"),
		}
	}
}
