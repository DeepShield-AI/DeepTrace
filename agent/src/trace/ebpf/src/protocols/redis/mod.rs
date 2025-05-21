use super::{utils::check_protocol, Infer};
use crate::structs::InferInfo;
use aya_ebpf::programs::TracePointContext;
use parse::redis;
use trace_common::{
	message::{Message, MessageType},
	protocols::L7Protocol,
	structs::Quintuple,
};

mod parse;
#[cfg(test)]
mod tests;
#[derive(Debug)]
pub(crate) struct Redis {
	pub first: u8,
	pub is_command: bool,
}

impl Redis {
	pub fn new() -> Self {
		Self { first: 0, is_command: false }
	}
	fn message_type(&self) -> MessageType {
		if self.first == b'*' && self.is_command {
			MessageType::Request
		} else {
			MessageType::Response
		}
	}
}

#[cfg(test)]
impl std::fmt::Display for Redis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&format!("Redis {{ first: {} }}", self.first as char))
	}
}

impl Infer for Redis {
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		_quintuple: Quintuple,
	) -> Result<Message, u32> {
		let payload = info.buf.as_slice();
		if info.len < 4 {
			return Err(0);
		}
		if !check_protocol(info.key, L7Protocol::Redis) {
			return Err(0);
		}
		match redis(payload, info.len) {
			Ok(redis) => {
				let mut message = Message::new();
				message.protocol = L7Protocol::Redis;
				message.type_ = redis.message_type();
				Ok(message)
			},
			Err(_) => Err(0_u32),
		}
	}
}
