use crate::protocols::L7Protocol;
use serde::Serialize;

#[derive(Default)]
pub struct Message {
	pub uuid: u32,
	pub seq: u32,
	pub protocol: L7Protocol,
	pub type_: MessageType,
}

impl Message {
	pub fn new() -> Self {
		Default::default()
	}
}

#[derive(Default, Clone, Copy, Serialize, Debug, PartialEq)]
pub enum MessageType {
	#[default]
	Unknown = -1,
	Request = 0,
	Response = 1,
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
