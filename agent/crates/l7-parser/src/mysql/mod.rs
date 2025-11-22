use crate::{Classification, Infer, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use com::Com;
use constants::*;
use ebpf_common::{
	alloc,
	error::{Result, code::*},
};
use observ_trace_common::{
	Buffer, Direction, L4Protocol, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
	maps::SOCKET_INFO,
};
use parse::mysql;

mod com;
mod constants;
mod parse;

pub(crate) struct MySQL {
	pub type_: MessageType,
}

impl MySQL {
	fn message_type(&self) -> MessageType {
		self.type_
	}
}
impl Infer for MySQL {
	#[inline(always)]
	fn parse(
		_ctx: &TracePointContext,
		quintuple: &Quintuple,
		direction: Direction,
		buffer: &Buffer<MAX_INFER_SIZE>,
		key: u64,
		enter_seq: u32,
		_exit_seq: u32,
		count: u32,
	) -> Result<Classification> {
		if quintuple.l4_protocol != L4Protocol::IPPROTO_TCP {
			return Err(MYSQL_L4_PROTOCOL_INVALID);
		}
		if !check_protocol(key, L7Protocol::MySQL) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}
		if buffer.len() <= MYSQL_HEADER_MIN_SIZE {
			return Err(INFER_PAYLOAD_TOO_SHORT);
		}

		mysql(buffer.as_slice(), count, direction)
			.or_else(|_| {
				unsafe { SOCKET_INFO.get(&key) }.ok_or(MAP_GET_FAILED).and_then(|socket_info| {
					if socket_info.prev_buf.len() > 0 &&
						socket_info.direction == direction &&
						socket_info.exit_seq == enter_seq
					{
						let buf = alloc::alloc_zero::<Buffer<MAX_INFER_SIZE>>()?;
						buf.append(socket_info.prev_buf.as_slice())?;
						buf.append(buffer.as_slice())?;
						mysql(buf.as_slice(), count + socket_info.prev_buf.len() as u32, direction)
					} else {
						Err(PARSE_MYSQL_FAILED)
					}
				})
			})
			.map(|mysql| {
				let mut classification = Classification::new();
				classification.protocol = L7Protocol::MySQL;
				classification.type_ = mysql.message_type();
				classification
			})
	}
}
