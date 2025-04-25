use super::{utils::check_protocol, Infer};
use crate::structs::InferInfo;
use aya_ebpf::programs::TracePointContext;
use constants::{DNS_HEADER_SIZE, DNS_MSG_MAX_SIZE};
use flag::PacketFlag;
use mercury_common::{
	message::{Message, MessageType},
	protocols::{L4Protocol, L7Protocol},
	structs::{Direction, Quintuple},
};
use opcode::OpCode;
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
pub(crate) struct DNS {
	/// The identification of the packet, must be defined when querying
	pub id: u16,
	/// Indicates the type of query in this packet
	pub opcode: OpCode,
	/// [RCode](RCode) indicates the response code for this packet
	pub response_code: RCode,

	pub z_flags: PacketFlag,
	pub questions: u16,
	pub answers: u16,
	pub name_servers: u16,
	pub additional_records: u16,
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
	fn parse(
		_ctx: &TracePointContext,
		info: &InferInfo,
		quintuple: Quintuple,
	) -> Result<Message, u32> {
		if info.count < DNS_HEADER_SIZE || info.count > DNS_MSG_MAX_SIZE {
			return Err(0_u32)
		}
		if !check_protocol(info.key, L7Protocol::DNS) {
			return Err(0);
		}
		let tmp = info.buf.as_slice();
		let payload = if quintuple.l4_protocol == L4Protocol::IPPROTO_TCP {
			let length =
				u16::from_be_bytes(tmp.get(0..2).ok_or(0_u32)?.try_into().map_err(|_| 0_u32)?);
			let start = if length as u32 + 2 == info.count || info.direction == Direction::Egress {
				2_usize
			} else {
				0_usize
			};
			&tmp[start..]
		} else {
			tmp
		};
		match parse::dns_header(payload) {
			Ok(header) => Ok(Message { protocol: L7Protocol::DNS, type_: header.message_type() }),
			Err(_) => Err(0_u32),
		}
	}
}
