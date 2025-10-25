use super::{
	ROCKETMQ_ONEWAY_REQUEST, ROCKETMQ_REQUEST, ROCKETMQ_RESPONSE, ROCKETMQ_SERIALIZE_TYPE_JSON,
	ROCKETMQ_SERIALIZE_TYPE_ROCKETMQ, RocketMQ,
};
use ebpf_common::error::{Result, code::*};
use nom::{
	IResult, Parser,
	combinator::{map, verify},
	error::ErrorKind,
	number::streaming::{be_i8, be_i24, be_i32},
};
use observ_trace_common::MessageType;

fn length(i: &[u8]) -> IResult<&[u8], i32> {
	verify(be_i32, |&length| length >= 4 && length <= 0xFFFF).parse(i)
}

fn serialize_type(i: &[u8]) -> IResult<&[u8], i8> {
	verify(be_i8, |&serialize_type| {
		serialize_type == ROCKETMQ_SERIALIZE_TYPE_JSON ||
			serialize_type == ROCKETMQ_SERIALIZE_TYPE_ROCKETMQ
	})
	.parse(i)
}

fn header_length(i: &[u8]) -> IResult<&[u8], i32> {
	map(be_i24, |header_length| header_length & 0xFFFFFF).parse(i)
}

pub(super) fn rocketmq(i: &[u8], count: usize) -> Result<RocketMQ> {
	let mut parser = (length, serialize_type, header_length);
	let (i, (length, serialize_type, header_length)) =
		parser.parse(i).map_err(|_| PARSE_ROCKETMQ_FAILED)?;

	if header_length > length - 4 {
		return Err(ROCKETMQ_PAYLOAD_LENGTH_INVALID);
	}

	let mut message_type = MessageType::Unknown;
	match serialize_type {
		// there must be the following characters
		// {"code":0,"flag":1,"language":"","opaque":1,"serializeTypeCurrentRPC":"JSON","version":0}
		// in header data at least, total: 89B
		// TODO: don't hardcode 89 here
		ROCKETMQ_SERIALIZE_TYPE_JSON if header_length >= 89 => {
			// compressed judgement due to instruction limit
			if i[0] != b'{' || i[1] != b'"' || i[6] != b'"' || i[7] != b':' {
				return Err(ROCKETMQ_JSON_PARSE_FAILED);
			}
			if i[8] >= b'0' && i[8] <= b'4' && i[9] == b',' {
				message_type = MessageType::Response;
			}
			// judgement based on flag (no extFields ahead), and code maybe 10, 200, 2000, -1000
			if count >= 30 {
				for p in 18..=21 {
					if i[p - 1] == b':' && i[p] == b'1' {
						message_type = MessageType::Response
					}
				}
			}
		},
		ROCKETMQ_SERIALIZE_TYPE_ROCKETMQ => {
			// there must be code(2B), language(1B), version(2B), opaque(4B) and flag(4B)
			// in header data at least, total: 2 + 1 + 2 + 4 + 4 = 13B
			if header_length < 13 {
				return Err(ROCKETMQ_HEADER_LENGTH_INVALID);
			}
			let (_, flag) =
				be_i32::<_, (_, ErrorKind)>(&i[9..]).map_err(|_| ROCKETMQ_ROCKETMQ_PARSE_FAILED)?;
			message_type = match flag {
				ROCKETMQ_REQUEST | ROCKETMQ_ONEWAY_REQUEST => MessageType::Request,
				ROCKETMQ_RESPONSE => MessageType::Response,
				_ => MessageType::Unknown,
			};
		},
		_ => return Err(ROCKETMQ_TYPE_INVALID),
	}
	Ok(RocketMQ { type_: message_type })
}
