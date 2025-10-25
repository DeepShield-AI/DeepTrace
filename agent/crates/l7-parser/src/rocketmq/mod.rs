use crate::{Infer, types::Classification, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use constants::*;
use ebpf_common::error::{Result, code::*};
use observ_trace_common::{
	Buffer, Direction, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
};
use parser::rocketmq;

mod constants;
mod parser;

/// RocketMQ packet header structure
// TODO: add fields here
pub(crate) struct RocketMQ {
	type_: MessageType,
}

impl RocketMQ {
	pub fn message_type(&self) -> MessageType {
		self.type_
	}
}

impl Infer for RocketMQ {
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
		if buffer.len() < ROCKETMQ_HEADER_MIN_SIZE {
			return Err(INFER_PAYLOAD_TOO_SHORT);
		}
		if !check_protocol(key, L7Protocol::RocketMQ) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}
		let payload = buffer.as_slice();
		rocketmq(payload, buffer.len()).map(|rocketmq| {
			let mut classification = Classification::new();
			classification.protocol = L7Protocol::RocketMQ;
			classification.type_ = rocketmq.message_type();
			classification
		})
	}
}
