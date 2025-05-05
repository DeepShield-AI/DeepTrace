use super::Infer;
use crate::{
	constants::{mask::INFER_MASK, MAX_INFER_PAYLOAD_SIZE},
	maps::{INFER_BUFFER, SOCKET_INFO},
	structs::InferInfo,
};
use aya_ebpf::{helpers::r#gen::bpf_probe_read, programs::TracePointContext};
use core::cmp::min;
use trace_common::{
	message::{Message, MessageType},
	protocols::L7Protocol,
	structs::{Direction, Quintuple},
};

mod opcode;
mod parse;
use super::utils::check_protocol;
use opcode::OpCode;
use parse::mongodb_header;

pub(crate) const MONGODB_HEADER_SIZE: u32 = 16;
/// In general, each message consists of a standard message header followed by request-specific data.
#[derive(Debug)]
pub(crate) struct MongoDB {
	/// The total size of the message in bytes. This total includes the 4 bytes that holds the message length.
	pub message_length: i32,
	/// A client or database-generated identifier that uniquely identifies this message. For the case of client-generated messages (e.g. [OpQuery](OpCode::OpQuery) and [OpGetMore](OpCode::OpGetMore)), it will be returned in the `response_to` field of the [OpReply](OpCode::OpReply) message.
	pub request_id: i32,
	/// In the case of a message from the database, this will be the `request_id` taken from the [OpQuery](OpCode::OpQuery) or [OpGetMore](OpCode::OpGetMore) messages from the client.
	pub response_to: i32,
	/// Type of message.
	pub op_code: OpCode,
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
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		_quintuple: Quintuple,
	) -> Result<Message, u32> {
		let payload = info.buf.as_slice();
		if !check_protocol(info.key, L7Protocol::MongoDB) {
			return Err(0);
		}
		if info.count <= MONGODB_HEADER_SIZE && info.direction == Direction::Ingress {
			return Err(0);
		}
		if let Ok(header) = mongodb_header(payload, info.count) {
			let mut message = Message::new();
			message.protocol = L7Protocol::MongoDB;
			message.type_ = header.message_type();
			return Ok(message)
		}
		// If the message is not complete, we can try to parse it again with the previous message
		let key = info.key;
		let map = unsafe { &SOCKET_INFO };
		let socket_info = unsafe { map.get(&key) }.ok_or(0_u32)?;
		if socket_info.prev_len > 0 &&
			socket_info.direction == info.direction &&
			socket_info.exit_seq == info.enter_seq
		{
			let buf = unsafe { INFER_BUFFER.get_ptr_mut(0) }.ok_or(0_u32)?;
			let ptr = unsafe { &mut *buf };

			if unsafe {
				bpf_probe_read(
					ptr.as_mut_ptr() as *mut _,
					socket_info.prev_len & INFER_MASK,
					socket_info.prev_buf.as_ptr() as *const _,
				)
			} != 0
			{
				return Err(0);
			}

			let start =
				min((socket_info.prev_len & INFER_MASK) as usize, MAX_INFER_PAYLOAD_SIZE as usize);
			let ptr = &mut ptr[start..];
			if unsafe {
				bpf_probe_read(
					ptr.as_mut_ptr() as *mut _,
					ptr.len() as u32 & INFER_MASK,
					info.buf.as_ptr() as *const _,
				)
			} != 0
			{
				return Err(0);
			}
			return match mongodb_header(&unsafe { *buf }, info.count + socket_info.prev_len) {
				Ok(header) => {
					let mut message = Message::new();
					message.protocol = L7Protocol::MongoDB;
					message.type_ = header.message_type();
					Ok(message)
				},
				Err(_) => Err(0),
			}
		} else if socket_info.l7protocol == L7Protocol::MongoDB &&
			info.direction == Direction::Egress
		{
			// bson response
			let mut message = Message::new();
			message.protocol = L7Protocol::MongoDB;
			message.type_ = MessageType::Response;
			return Ok(message);
		}
		Err(0)
	}
}
