use super::{
	constants::MAX_RECOURSE_RECORD_NUM,
	flag::{masks, PacketFlag},
	opcode::OpCode,
	rcode::RCode,
	DNS,
};
use nom::{combinator::verify, number::streaming::be_u16, IResult, Parser};

fn flags(i: &[u8]) -> IResult<&[u8], u16> {
	verify(be_u16, |flags: &u16| (flags & masks::RESERVED_MASK) == 0).parse(i)
}

pub(super) fn dns_header(i: &[u8]) -> Result<DNS, u32> {
	let mut header = (be_u16, flags, be_u16, be_u16, be_u16, be_u16);
	let (_, (id, flags, questions, answers, name_servers, additional_records)) =
		header.parse(i).map_err(|_| 0_u32)?;
	// TODO: why use 11?
	if !(0 < questions && questions < 11) {
		return Err(0_u32);
	}
	if (questions + answers + name_servers + additional_records) > MAX_RECOURSE_RECORD_NUM {
		return Err(0_u32);
	}
	let opcode =
		OpCode::try_from((flags & masks::OPCODE_MASK) >> masks::OPCODE_MASK.trailing_zeros())
			.map_err(|_| 0_u32)?;

	if opcode != OpCode::StandardQuery {
		return Err(0_u32);
	}
	let response_code = RCode::try_from(flags & masks::RESPONSE_CODE_MASK).map_err(|_| 0_u32)?;
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
