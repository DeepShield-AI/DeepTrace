use crate::{Classification, Infer, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use constants::CASSANDRA_MIN_SIZE;
use ebpf_common::error::{Result, code::*};
use flag::Flags;
use observ_trace_common::{
	Buffer, Direction, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
};
use opcode::OpCode;
use parse::cassandra;

mod constants;
mod flag;
mod opcode;
mod parse;

/// Cassandra packet header
/// ```markdown
/// 0         8        16        24        32         40
/// +---------+---------+---------+---------+---------+
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
	type_: MessageType,
	version: u8,
	flags: Flags,
	stream: i16,
	opcode: OpCode,
	length: u32,
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
	#[inline(always)]
	fn parse(
		_ctx: &TracePointContext,
		_quintuple: &Quintuple,
		_direction: Direction,
		buffer: &Buffer<MAX_INFER_SIZE>,
		key: u64,
		_enter_seq: u32,
		_exit_seq: u32,
		_count: u32,
	) -> Result<Classification> {
		if buffer.len() < CASSANDRA_MIN_SIZE {
			return Err(INFER_PAYLOAD_TOO_SHORT);
		}
		if !check_protocol(key, L7Protocol::Cassandra) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}
		let payload = buffer.as_slice();
		cassandra(payload).map(|cassandra| {
			let mut classification = Classification::new();
			classification.protocol = L7Protocol::Cassandra;
			classification.type_ = cassandra.message_type();
			classification
		})
	}
}
