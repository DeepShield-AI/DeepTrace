use super::{check_protocol, parse::mysql_header, Infer, MYSQL_HEADER_MIN_SIZE};
use crate::{
	constants::{mask::INFER_MASK, MAX_INFER_PAYLOAD_SIZE},
	maps::{INFER_BUFFER, SOCKET_INFO},
	structs::InferInfo,
};
use aya_ebpf::{helpers::gen::bpf_probe_read, programs::TracePointContext};
use core::cmp::min;
use trace_common::{
	message::{Message, MessageType},
	protocols::{L4Protocol, L7Protocol},
	structs::Quintuple,
};
/// In general, each message consists of a standard message header followed by request-specific data.
#[derive(Debug)]
pub(crate) struct MySQL {
	pub type_: MessageType,
}

impl MySQL {
	fn message_type(&self) -> MessageType {
		self.type_
	}
}
impl Infer for MySQL {
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		quintuple: Quintuple,
	) -> Result<Message, u32> {
		if quintuple.l4_protocol != L4Protocol::IPPROTO_TCP {
			return Err(0);
		}
		if !check_protocol(info.key, L7Protocol::MySQL) {
			return Err(0);
		}
		if info.count <= MYSQL_HEADER_MIN_SIZE {
			return Err(0);
		}
		let payload = info.buf.as_slice();
		if let Ok(header) = mysql_header(payload, info.count, info.direction) {
			let mut message = Message::new();
			message.protocol = L7Protocol::MySQL;
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
			return match mysql_header(
				&unsafe { *buf },
				info.count + socket_info.prev_len,
				info.direction,
			) {
				Ok(header) => {
					let mut message = Message::new();
					message.protocol = L7Protocol::MySQL;
					message.type_ = header.message_type();
					Ok(message)
				},
				Err(_) => Err(0),
			}
		}
		Err(0)
	}
}
