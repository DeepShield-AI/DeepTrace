use super::{check_protocol, parse::cassandra_header, Flags, Infer, OpCode, CASSANDRA_MIN_SIZE};
use crate::structs::InferInfo;
use aya_ebpf::programs::TracePointContext;
use trace_common::{
	message::{Message, MessageType},
	protocols::L7Protocol,
	structs::Quintuple,
};

/// Cassandra packet header structure
/// ```markdown
/// 0         8        16        24        32         40
///	+---------+---------+---------+---------+---------+
/// | version |  flags  |      stream       | opcode  |
/// +---------+---------+---------+---------+---------+
/// |                length                 |
/// +---------+---------+---------+---------+
/// |                                       |
/// .            ...  body ...              .
/// .                                       .
/// .                                       .
/// +----------------------------------------
/// ```
pub(crate) struct Cassandra {
	pub type_: MessageType,
	pub version: u8,
	pub flags: Flags,
	pub stream: i16,
	pub opcode: OpCode,
	pub length: u32,
}

impl Cassandra {
	pub fn message_type(&self) -> MessageType {
		match self.opcode {
			OpCode::STARTUP |
			OpCode::AUTH_RESPONSE |
			OpCode::OPTIONS |
			OpCode::QUERY |
			OpCode::PREPARE |
			OpCode::EXECUTE |
			OpCode::BATCH |
			OpCode::REGISTER
				if self.type_ == MessageType::Request =>
				MessageType::Request,
			OpCode::ERROR |
			OpCode::READY |
			OpCode::AUTHENTICATE |
			OpCode::SUPPORTED |
			OpCode::RESULT |
			OpCode::EVENT |
			OpCode::AUTH_CHALLENGE |
			OpCode::AUTH_SUCCESS
				if self.type_ == MessageType::Response =>
				MessageType::Response,
			_ => MessageType::Unknown,
		}
	}
}

impl Infer for Cassandra {
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		_quintuple: Quintuple,
	) -> Result<Message, u32> {
		if info.count < CASSANDRA_MIN_SIZE {
			return Err(0);
		}
		if !check_protocol(info.key, L7Protocol::Cassandra) {
			return Err(0);
		}
		let payload = info.buf.as_slice();
		match cassandra_header(payload) {
			Ok(header) => {
				let mut message = Message::new();
				message.protocol = L7Protocol::Cassandra;
				message.type_ = header.message_type();
				return Ok(message);
			},
			Err(e) => {
				return Err(e);
			},
		}
	}
}
