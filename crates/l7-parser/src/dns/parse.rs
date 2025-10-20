use super::{
	DNS,
	constants::MAX_RECOURSE_RECORD_NUM,
	flag::{PacketFlag, masks},
	opcode::OpCode,
	rcode::RCode,
};
use ebpf_common::error::{Result, code::*};
use nom::{IResult, Parser, combinator::verify, number::streaming::be_u16};

fn flags(i: &[u8]) -> IResult<&[u8], u16> {
	verify(be_u16, |flags: &u16| (flags & masks::RESERVED_MASK) == 0).parse(i)
}

pub(super) fn dns(i: &[u8]) -> Result<DNS> {
	let mut header = (be_u16, flags, be_u16, be_u16, be_u16, be_u16);
	let (_, (id, flags, questions, answers, name_servers, additional_records)) =
		header.parse(i).map_err(|_| PARSE_DNS_FAILED)?;
	// TODO: why use 11?
	if !(0 < questions && questions < 11) {
		return Err(DNS_QUESTION_NUM_INVALID);
	}
	if (questions + answers + name_servers + additional_records) > MAX_RECOURSE_RECORD_NUM {
		return Err(DNS_RECOURSE_RECORD_NUM_INVALID);
	}
	let opcode =
		OpCode::try_from((flags & masks::OPCODE_MASK) >> masks::OPCODE_MASK.trailing_zeros())
			.map_err(|_| DNS_OPCODE_PARSE_FAILED)?;

	if opcode != OpCode::StandardQuery {
		return Err(DNS_NOT_STANDARD_QUERY);
	}
	let response_code =
		RCode::try_from(flags & masks::RESPONSE_CODE_MASK).map_err(|_| DNS_RCODE_PARSE_FAILED)?;
	let z_flags = PacketFlag::from_bits_truncate(flags);
	Ok(DNS {
		id,
		opcode,
		response_code,
		z_flags,
		questions,
		answers,
		name_servers,
		additional_records,
	})
}
