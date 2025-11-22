use observ_trace_common::{L7Protocol, MessageType};

#[repr(C)]
pub struct Classification {
	pub uuid: u64,
	pub seq: u32,
	pub protocol: L7Protocol,
	pub type_: MessageType,
}

impl Default for Classification {
	fn default() -> Self {
		Self { uuid: 0, seq: 0, protocol: L7Protocol::Unknown, type_: MessageType::Unknown }
	}
}

impl Classification {
	pub fn new() -> Self {
		Self::default()
	}
}
