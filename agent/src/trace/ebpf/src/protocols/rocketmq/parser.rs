use super::{
	RocketMQ, ROCKETMQ_ONEWAY_REQUEST, ROCKETMQ_REQUEST, ROCKETMQ_RESPONSE,
	ROCKETMQ_SERIALIZE_TYPE_JSON, ROCKETMQ_SERIALIZE_TYPE_ROCKETMQ,
};
use nom::{
	combinator::{map, verify}, error::ErrorKind, number::streaming::{be_i24, be_i32, be_i8}, IResult, Parser
};
use trace_common::message::MessageType;
fn length(i: &[u8]) -> IResult<&[u8], i32> {
	verify(be_i32, |&length| length >= 4 && length <= 65535).parse(i)
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

pub(super) fn rocketmq_header(i: &[u8], count: u32) -> Result<RocketMQ, u32> {
	let mut parser = (length, serialize_type, header_length);
	let (i, (length, serialize_type, header_length)) = parser.parse(i).map_err(|_| 0_u32)?;

	if header_length > length - 4 {
		return Err(0_u32);
	}

	let mut message_type = MessageType::Unknown;
	match serialize_type {
		// there must be the following characters
		// {"code":0,"flag":1,"language":"","opaque":1,"serializeTypeCurrentRPC":"JSON","version":0}
		// in header data at least, total: 89B
		ROCKETMQ_SERIALIZE_TYPE_JSON if header_length >= 89 => {
			// compressed judgement due to instruction limit
			if i[0] != b'{' || i[1] != b'"' || i[6] != b'"' || i[7] != b':' {
				return Err(0_u32);
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
				return Err(0_u32);
			}
			let (_, flag) = be_i32::<_, (_, ErrorKind)>(&i[9..]).map_err(|_| 0_u32)?;
			message_type = match flag {
				ROCKETMQ_REQUEST | ROCKETMQ_ONEWAY_REQUEST => MessageType::Request,
				ROCKETMQ_RESPONSE => MessageType::Response,
				_ => MessageType::Unknown,
			};
		},
		_ => return Err(0_u32),
	}
	Ok(RocketMQ { type_: message_type })
}
