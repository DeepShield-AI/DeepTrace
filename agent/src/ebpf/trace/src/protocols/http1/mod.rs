use super::{utils::check_protocol, Infer};
use crate::structs::InferInfo;
use aya_ebpf::programs::TracePointContext;
use constants::HTTP1_MIN_SIZE;
use parse::http1;
use trace_common::{
	message::{Message, MessageType},
	protocols::L7Protocol,
	structs::Quintuple,
};
mod constants;
mod parse;

pub(crate) struct HTTP1 {
	pub type_: MessageType,
}

impl HTTP1 {
	pub fn message_type(&self) -> MessageType {
		self.type_
	}
}

impl Infer for HTTP1 {
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		_quintuple: Quintuple,
	) -> Result<Message, u32> {
		if info.count < HTTP1_MIN_SIZE {
			return Err(0_u32)
		}
		if !check_protocol(info.key, L7Protocol::HTTP1) {
			return Err(0);
		}
		let payload = info.buf.as_slice();
		match http1(payload) {
			Ok(header) => {
				let mut message = Message::new();
				message.protocol = L7Protocol::HTTP1;
				message.type_ = header.message_type();
				return Ok(message);
			},
			Err(_) => Err(0_u32),
		}
	}
}
