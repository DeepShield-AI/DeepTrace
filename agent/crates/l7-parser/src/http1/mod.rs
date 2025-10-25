use crate::{Infer, types::Classification, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use constants::HTTP1_MIN_SIZE;
use ebpf_common::{
	alloc,
	error::{Result, code::*},
};
use observ_trace_common::{
	Buffer, Direction, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
	maps::SOCKET_INFO,
};
use parse::http1;
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
	#[inline(always)]
	fn parse(
		_ctx: &TracePointContext,
		_quintuple: &Quintuple,
		direction: Direction,
		buffer: &Buffer<MAX_INFER_SIZE>,
		key: u64,
		enter_seq: u32,
		_exit_seq: u32,
	) -> Result<Classification> {
		if buffer.len() < HTTP1_MIN_SIZE {
			return Err(INFER_PAYLOAD_TOO_SHORT);
		}
		if !check_protocol(key, L7Protocol::HTTP1) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}
		http1(buffer.as_slice())
			.or_else(|_| {
				unsafe { SOCKET_INFO.get(&key) }.ok_or(MAP_GET_FAILED).and_then(|socket_info| {
					if socket_info.prev_buf.len() > 0 &&
						socket_info.direction == direction &&
						socket_info.exit_seq == enter_seq
					{
						let buf = alloc::alloc_zero::<Buffer<MAX_INFER_SIZE>>()?;
						buf.append(socket_info.prev_buf.as_slice())?;
						buf.append(buffer.as_slice())?;
						http1(buf.as_slice())
					} else {
						Err(PARSE_HTTP1_FAILED)
					}
				})
			})
			.map(|header| {
				let mut classification = Classification::new();
				classification.protocol = L7Protocol::HTTP1;
				classification.type_ = header.message_type();
				classification
			})
	}
}
