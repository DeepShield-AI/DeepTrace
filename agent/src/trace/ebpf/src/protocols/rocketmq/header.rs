use super::{check_protocol, parser::rocketmq_header, Infer, ROCKETMQ_HEADER_MIN_SIZE};
use crate::structs::InferInfo;
use aya_ebpf::programs::TracePointContext;
use trace_common::{
	message::{Message, MessageType},
	protocols::L7Protocol,
	structs::Quintuple,
};

/// RocketMQ packet header structure
/// ```markdown
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
pub(crate) struct RocketMQ {
	pub type_: MessageType,
}

impl RocketMQ {
	pub fn message_type(&self) -> MessageType {
		self.type_
	}
}

impl Infer for RocketMQ {
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		_quintuple: Quintuple,
	) -> Result<Message, u32> {
		if info.count < ROCKETMQ_HEADER_MIN_SIZE {
			return Err(0_u32);
		}
		if !check_protocol(info.key, L7Protocol::RocketMQ) {
			return Err(0);
		}
		let payload = info.buf.as_slice();
		match rocketmq_header(payload, info.len) {
			Ok(header) => {
				let mut message = Message::new();
				message.protocol = L7Protocol::RocketMQ;
				message.type_ = header.message_type();
				Ok(message)
			},
			Err(_) => Err(0_u32),
		}
	}
}
