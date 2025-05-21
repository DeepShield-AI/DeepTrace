use super::{
	check_protocol, parse::memcached_header, Infer, OpCode, BINARY_PROTOCOL_REQUEST,
	BINARY_PROTOCOL_RESPONSE, HEADER_SIZE,
};
use crate::structs::InferInfo;
use aya_ebpf::programs::TracePointContext;
use trace_common::{
	message::{Message, MessageType},
	protocols::L7Protocol,
	structs::Quintuple,
};

/// Request Header
///```markdown
/// Byte/     0       |       1       |       2       |       3       |
///    /              |               |               |               |
///   |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|
///   +---------------+---------------+---------------+---------------+
///  0| Magic         | Opcode        | Key length                    |
///   +---------------+---------------+---------------+---------------+
///  4| Extras length | Data type     | vbucket id                    |
///   +---------------+---------------+---------------+---------------+
///  8| Total body length                                             |
///   +---------------+---------------+---------------+---------------+
/// 12| Opaque                                                        |
///   +---------------+---------------+---------------+---------------+
/// 16| CAS                                                           |
///   |                                                               |
///   +---------------+---------------+---------------+---------------+
/// Total 24 bytes
/// ```
/// Response Header
/// ```markdown
/// Byte/     0       |       1       |       2       |       3       |
///    /              |               |               |               |
///   |0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|0 1 2 3 4 5 6 7|
///   +---------------+---------------+---------------+---------------+
///  0| Magic         | Opcode        | Key Length                    |
///   +---------------+---------------+---------------+---------------+
///  4| Extras length | Data type     | Status                        |
///   +---------------+---------------+---------------+---------------+
///  8| Total body length                                             |
///   +---------------+---------------+---------------+---------------+
/// 12| Opaque                                                        |
///   +---------------+---------------+---------------+---------------+
/// 16| CAS                                                           |
///   |                                                               |
///   +---------------+---------------+---------------+---------------+
/// Total 24 bytes
/// ```
#[repr(C)]
#[derive(Debug)]
pub(crate) struct Memcached {
	pub magic: u8,
	pub opcode: OpCode,
	pub key_length: u16,
	pub extras_length: u8,
	pub data_type: u8,
	pub field: u16,
	pub total_body_length: u32,
	pub opaque: u32,
	pub cas: u64,
}

impl Memcached {
	pub fn message_type(&self) -> MessageType {
		match self.magic {
			BINARY_PROTOCOL_REQUEST => MessageType::Request,
			BINARY_PROTOCOL_RESPONSE => MessageType::Response,
			_ => MessageType::Unknown,
		}
	}
}

impl Infer for Memcached {
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		_quintuple: Quintuple,
	) -> Result<Message, u32> {
		if info.count < HEADER_SIZE as u32 {
			return Err(0)
		}
		if !check_protocol(info.key, L7Protocol::Memcached) {
			return Err(0);
		}
		let payload = info.buf.as_slice();
		match memcached_header(payload) {
			Ok(memcached) => {
				let mut message = Message::new();
				message.protocol = L7Protocol::Memcached;
				message.type_ = memcached.message_type();
				Ok(message)
			},
			Err(_) => Err(0),
		}
	}
}
