use crate::{Classification, Infer, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use ebpf_common::error::{Result, code::*};
use kind::Kind;
use observ_trace_common::{
	Buffer, Direction, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
};
use parse::{binary_thrift, compact_thrift};

mod constants;
mod kind;
mod parse;

pub(crate) struct Thrift {
	kind: Kind,
}

impl Thrift {
	fn message_type(&self) -> MessageType {
		match self.kind {
			Kind::Call | Kind::Oneway => MessageType::Request,
			Kind::Reply | Kind::Exception => MessageType::Response,
		}
	}
}

impl Infer for Thrift {
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
		if !check_protocol(key, L7Protocol::Thrift) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}
		let payload = buffer.as_slice();

		binary_thrift(payload, buffer.len())
			.or_else(|_| compact_thrift(payload))
			.map(|thrift| {
				let mut classification = Classification::new();
				classification.protocol = L7Protocol::Thrift;
				classification.type_ = thrift.message_type();
				classification
			})
	}
}
