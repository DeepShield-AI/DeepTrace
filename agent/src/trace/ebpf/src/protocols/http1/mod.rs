use super::{utils::check_protocol, Infer};
use crate::{
	constants::{mask::INFER_MASK, MAX_INFER_PAYLOAD_SIZE},
	maps::{INFER_BUFFER, SOCKET_INFO},
	structs::InferInfo,
};
use aya_ebpf::{helpers::gen::bpf_probe_read, programs::TracePointContext};
use constants::HTTP1_MIN_SIZE;
use core::cmp::min;
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
		if let Ok(header) = http1(payload) {
			let mut message = Message::new();
			message.protocol = L7Protocol::HTTP1;
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
			return match http1(&unsafe { *buf }) {
				Ok(header) => {
					let mut message = Message::new();
					message.protocol = L7Protocol::HTTP1;
					message.type_ = header.message_type();
					Ok(message)
				},
				Err(_) => Err(0),
			}
		}
		Err(0)
	}
}
