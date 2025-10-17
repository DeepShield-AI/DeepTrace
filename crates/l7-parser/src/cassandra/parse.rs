use super::{Cassandra, Flags, OpCode};
use core::mem;
use nom::{
	IResult, Parser,
	combinator::{map, map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::{be_i16, be_u8, be_u32},
};
use observ_trace_common::message::MessageType;

fn type_and_version(i: &[u8]) -> IResult<&[u8], (MessageType, u8)> {
	verify(
		map(be_u8, |type_and_version| {
			(unsafe { mem::transmute((type_and_version & 0xF0) >> 4) }, type_and_version & 0xF)
		}),
		|&(_, version)| version == 3 || version == 4 || version == 5,
	)
	.parse(i)
}
fn flags(i: &[u8]) -> IResult<&[u8], Flags> {
	map_res(be_u8, |flags: u8| Flags::try_from(flags).map_err(|_| Error::new(i, ErrorKind::MapRes)))
		.parse(i)
}

fn stream(i: &[u8]) -> IResult<&[u8], i16> {
	verify(be_i16, |&stream| stream >= -1).parse(i)
}

fn op_code(i: &[u8]) -> IResult<&[u8], OpCode> {
	map_res(be_u8, |opcode: u8| {
		OpCode::try_from(opcode).map_err(|_| Error::new(i, ErrorKind::MapRes))
	})
	.parse(i)
}

pub(super) fn cassandra_header(i: &[u8]) -> Result<Cassandra, u32> {
	let mut header = (type_and_version, flags, stream, op_code, be_u32);
	let (_, ((type_, version), flags, stream, opcode, length)) =
		header.parse(i).map_err(|_| 0_u32)?;

	Ok(Cassandra { type_, version, flags, stream, opcode, length })
}
