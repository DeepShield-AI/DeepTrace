use crate::{Classification, Infer, utils::check_protocol};
use aya_ebpf::programs::TracePointContext;
use constants::*;
use ebpf_common::error::{Result, code::*};
use flag::PacketFlag;
use observ_trace_common::{
	Buffer, Direction, L4Protocol, L7Protocol, MessageType, Quintuple, constants::MAX_INFER_SIZE,
};
use opcode::OpCode;
use parse::dns;
use rcode::RCode;

mod constants;
mod flag;
mod opcode;
mod parse;
mod rcode;

/// DNS packet header structure
/// ```markdown
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[allow(clippy::upper_case_acronyms)]
pub(crate) struct DNS {
	/// The identification of the packet, must be defined when querying
	id: u16,
	/// Indicates the type of query in this packet
	opcode: OpCode,
	/// [RCode] indicates the response code for this packet
	response_code: RCode,

	z_flags: PacketFlag,
	questions: u16,
	answers: u16,
	name_servers: u16,
	additional_records: u16,
}

impl DNS {
	pub fn message_type(&self) -> MessageType {
		if self.z_flags.contains(PacketFlag::Response) {
			MessageType::Response
		} else {
			MessageType::Request
		}
	}
}

impl Infer for DNS {
	#[inline(always)]
	fn parse(
		_ctx: &TracePointContext,
		quintuple: &Quintuple,
		direction: Direction,
		buffer: &Buffer<MAX_INFER_SIZE>,
		key: u64,
		_enter_seq: u32,
		_exit_seq: u32,
	) -> Result<Classification> {
		if buffer.len() < DNS_HEADER_SIZE || buffer.len() > DNS_MSG_MAX_SIZE {
			return Err(INFER_PAYLOAD_LENGTH_INVALID);
		}
		if !check_protocol(key, L7Protocol::DNS) {
			return Err(SOCKET_PROTOCOL_MISMATCH);
		}

		let tmp = buffer.as_slice();
		let payload = if quintuple.l4_protocol == L4Protocol::IPPROTO_TCP {
			let length =
				u16::from_be_bytes(tmp.get(0..2).ok_or(0_u32)?.try_into().map_err(|_| 0_u32)?);
			let start = if length as usize + 2 == buffer.len() || direction == Direction::Egress {
				2_usize
			} else {
				0_usize
			};
			&tmp[start..]
		} else {
			tmp
		};

		dns(payload).map(|dns| {
			let mut classification = Classification::new();
			classification.protocol = L7Protocol::DNS;
			classification.type_ = dns.message_type();
			classification
		})
	}
}
