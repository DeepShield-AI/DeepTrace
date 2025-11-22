use crate::{Classification, Infer, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use ebpf_common::error::{Result, code::*};
use observ_trace_common::{
	Buffer, Direction, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
};
use parse::redis;

mod parse;

pub(crate) struct Redis {
	first: u8,
	is_command: bool,
}

impl Redis {
	pub fn new() -> Self {
		Self { first: 0, is_command: false }
	}
	fn message_type(&self) -> MessageType {
		if self.first == b'*' && self.is_command {
			MessageType::Request
		} else {
			MessageType::Response
		}
	}
}

impl Infer for Redis {
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
		let payload = buffer.as_slice();
		if buffer.len() < 4 {
			return Err(INFER_PAYLOAD_TOO_SHORT);
		}
		if !check_protocol(key, L7Protocol::Redis) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}
		redis(payload, buffer.len()).map(|redis| {
			let mut classification = Classification::new();
			classification.protocol = L7Protocol::Redis;
			classification.type_ = redis.message_type();
			classification
		})
	}
}
