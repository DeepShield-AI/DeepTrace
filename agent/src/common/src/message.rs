use crate::protocols::L7Protocol;

#[derive(Default, Clone, Copy, Debug)]
pub enum MessageType {
	#[default]
	Unknown,
	Request,
	Response,
}

#[derive(Default)]
pub struct Message {
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
