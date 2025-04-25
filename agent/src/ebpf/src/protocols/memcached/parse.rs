//! Parse memcached binary protocol.
use super::{Memcached, OpCode, BINARY_PROTOCOL_REQUEST, BINARY_PROTOCOL_RESPONSE};
use core::slice;
use nom::{
	branch::alt,
	bytes::streaming::tag,
	combinator::{map, map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::{be_u16, be_u32, be_u64, be_u8},
	IResult, Parser,
};

fn request_magic(i: &[u8]) -> IResult<&[u8], u8> {
	map(tag(slice::from_ref(&BINARY_PROTOCOL_REQUEST)), |magic: &[u8]| magic[0]).parse(i)
}

fn response_magic(i: &[u8]) -> IResult<&[u8], u8> {
	map(tag(slice::from_ref(&BINARY_PROTOCOL_RESPONSE)), |magic: &[u8]| magic[0]).parse(i)
}

fn opcode(i: &[u8]) -> IResult<&[u8], OpCode> {
	map_res(be_u8, |opcode: u8| {
		OpCode::try_from(opcode).map_err(|_| Error::new(i, ErrorKind::MapRes))
	})
	.parse(i)
}

fn status(i: &[u8]) -> IResult<&[u8], u16> {
	verify(be_u16, |status: &u16| matches!(status, 0x00..=0x09 | 0x81..=0x86)).parse(i)
}

fn data_type(i: &[u8]) -> IResult<&[u8], &[u8]> {
	tag(slice::from_ref(&0_u8)).parse(i)
}

#[inline]
pub(super) fn memcached_header(i: &[u8]) -> Result<Memcached, u32> {
	let request_header =
		(request_magic, opcode, be_u16, be_u8, data_type, be_u16, be_u32, be_u32, be_u64);
	let response_header =
		(response_magic, opcode, be_u16, be_u8, data_type, status, be_u32, be_u32, be_u64);
	let mut header = alt((request_header, response_header));
	let (_, (magic, opcode, key_length, extras_length, _, field, total_body_length, opaque, cas)) =
		header.parse(i).map_err(|_| 0_u32)?;
	Ok(Memcached {
		magic,
		opcode,
		key_length,
		extras_length,
		data_type: 0,
		field,
		total_body_length,
		opaque,
		cas,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::tests::load_pcap;
	use mercury_common::consts::MAX_PAYLOAD_SIZE;
	const FILE_DIR: &str = "../../../tests/protocols/memcached";
	#[test]
	fn test_memcached_pcap() -> Result<(), u32> {
		let files = vec![("memcached.pcap", "memcached.result")];
		for (actual, expected) in files {
			let actual = format!("{}/{}", FILE_DIR, actual);
			let expected = format!("{}/{}", FILE_DIR, expected);
			let expected = std::fs::read_to_string(&expected).map_err(|_| 0_u32)?;
			let actual = run(&actual).map_err(|_| 0_u32)?;
			assert_eq!(actual, expected, "{} != {}", actual, expected);
		}
		Ok(())
	}

	fn run(actual: &str) -> Result<String, u32> {
		let packets = load_pcap(actual, MAX_PAYLOAD_SIZE as usize)?;
		if packets.is_empty() {
			return Err(0);
		}
		let mut output = String::new();
		for (_, data) in packets {
			let Ok(header) = memcached_header(&data) else {
				continue;
			};
			output.push_str(&format!("{:?}, {:?}\n", header.message_type(), header));
		}
		Ok(output)
	}
}
