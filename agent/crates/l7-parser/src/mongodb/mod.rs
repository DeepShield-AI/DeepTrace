use crate::{Classification, Infer, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use constants::*;
use ebpf_common::{
	alloc,
	error::{Result, code::*},
};
use observ_trace_common::{
	Buffer, Direction, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
	maps::SOCKET_INFO,
};
use opcode::OpCode;
use parse::mongodb;

mod constants;
mod opcode;
mod parse;

/// In general, each message consists of a standard message header followed by request-specific data.
pub(crate) struct MongoDB {
	/// The total size of the message in bytes. This total includes the 4 bytes that holds the message length.
	message_length: i32,
	/// A client or database-generated identifier that uniquely identifies this message. For the case of client-generated messages (e.g. [OpQuery](OpCode::OpQuery) and [OpGetMore](OpCode::OpGetMore)), it will be returned in the `response_to` field of the [OpReply](OpCode::OpReply) message.
	request_id: i32,
	/// In the case of a message from the database, this will be the `request_id` taken from the [OpQuery](OpCode::OpQuery) or [OpGetMore](OpCode::OpGetMore) messages from the client.
	response_to: i32,
	/// Type of message.
	op_code: OpCode,
}

impl MongoDB {
	fn message_type(&self) -> MessageType {
		match self.op_code {
			OpCode::OpUpdate |
			OpCode::OpInsert |
			OpCode::OpQuery |
			OpCode::OpGetMore |
			OpCode::OpDelete |
			OpCode::OpKillCursors |
			OpCode::OpCompressed |
			OpCode::Reserved => MessageType::Request,
			OpCode::OpReply => MessageType::Response,
			OpCode::OpMsg => match self.response_to {
				0 => MessageType::Request,
				_ => MessageType::Response,
			},
		}
	}
}
impl Infer for MongoDB {
	#[inline(always)]
	fn parse(
		_ctx: &TracePointContext,
		_quintuple: &Quintuple,
		direction: Direction,
		buffer: &Buffer<MAX_INFER_SIZE>,
		key: u64,
		enter_seq: u32,
		_exit_seq: u32,
		count: u32,
	) -> Result<Classification> {
		if buffer.len() <= MONGODB_HEADER_SIZE && direction == Direction::Ingress {
			return Err(INFER_PAYLOAD_TOO_SHORT);
		}
		if !check_protocol(key, L7Protocol::MongoDB) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}

		mongodb(buffer.as_slice(), count)
			.or_else(|_| {
				unsafe { SOCKET_INFO.get(&key) }.ok_or(MAP_GET_FAILED).and_then(|socket_info| {
					if socket_info.prev_buf.len() > 0 &&
						socket_info.direction == direction &&
						socket_info.exit_seq == enter_seq
					{
						let buf = alloc::alloc_zero::<Buffer<MAX_INFER_SIZE>>()?;
						buf.append(socket_info.prev_buf.as_slice())?;
						buf.append(buffer.as_slice())?;
						mongodb(buf.as_slice(), count + socket_info.prev_buf.len() as u32)
					} else if socket_info.l7protocol == L7Protocol::MongoDB &&
						direction == Direction::Egress
					{
						Ok(MongoDB {
							message_length: 0,
							request_id: 0,
							response_to: 1, // non-zero to indicate response
							op_code: OpCode::OpMsg,
						})
					} else {
						Err(PARSE_MONGODB_FAILED)
					}
				})
			})
			.map(|mongodb| {
				let mut classification = Classification::new();
				classification.protocol = L7Protocol::MongoDB;
				classification.type_ = mongodb.message_type();
				classification
			})
	}
}
