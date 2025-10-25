use crate::{Classification, Infer, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use constants::*;
use ebpf_common::error::{
	Result,
	code::{INFER_PAYLOAD_TOO_SHORT, SOCKET_PROTOCOL_MISMATCH},
};
use observ_trace_common::{
	Buffer, Direction, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
};
use opcode::OpCode;
use parse::memcached;

mod constants;
mod opcode;
mod parse;
mod status;

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
pub(crate) struct Memcached {
	magic: u8,
	opcode: OpCode,
	key_length: u16,
	extras_length: u8,
	data_type: u8,
	field: u16,
	total_body_length: u32,
	opaque: u32,
	cas: u64,
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
	#[inline(always)]
	fn parse(
		_ctx: &TracePointContext,
		_quintuple: &Quintuple,
		_direction: Direction,
		buffer: &Buffer<MAX_INFER_SIZE>,
		key: u64,
		_enter_seq: u32,
		_exit_seq: u32,
	) -> Result<Classification> {
		if buffer.len() < MEMCACHED_HEADER_SIZE {
			return Err(INFER_PAYLOAD_TOO_SHORT);
		}
		if !check_protocol(key, L7Protocol::Memcached) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}
		let payload = buffer.as_slice();
		memcached(payload).map(|memcached| {
			let mut classification = Classification::new();
			classification.protocol = L7Protocol::Memcached;
			classification.type_ = memcached.message_type();
			classification
		})
	}
}
