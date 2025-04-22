use super::{MongoDB, OpCode, HEADER_SIZE};
use nom::{
	combinator::{map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::le_i32,
	IResult, Parser,
};

fn message_length(expected: u32) -> impl Fn(&[u8]) -> IResult<&[u8], i32> {
	move |i| {
		verify(le_i32, |&message_length| {
			// message_length as u32 == expected || message_length as u32 & 0xFFFFFF00 >= expected
			true
		})
		.parse(i)
	}
}

fn request_id(i: &[u8]) -> IResult<&[u8], i32> {
	verify(le_i32, |&request_id| request_id >= 0).parse(i)
}

fn response_to(i: &[u8]) -> IResult<&[u8], i32> {
	verify(le_i32, |&response_to| response_to >= 0).parse(i)
}

fn opcode(i: &[u8]) -> IResult<&[u8], OpCode> {
	map_res(le_i32, |opcode: i32| {
		OpCode::try_from(opcode).map_err(|_| Error::new(i, ErrorKind::MapRes))
	})
	.parse(i)
}

pub(super) fn mongodb_header(i: &[u8], count: u32) -> Result<MongoDB, u32> {
	if count < HEADER_SIZE as u32 {
		return Err(0);
	}
	let mut header = (message_length(count), request_id, response_to, opcode);
	let (_, (message_length, request_id, response_to, op_code)) =
		header.parse(i).map_err(|_| 0_u32)?;
	Ok(MongoDB { message_length, request_id, response_to, op_code })
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::tests::load_pcap;
	use mercury_common::consts::MAX_PAYLOAD_SIZE;
	// the binary file is in target/debug/deps, so use ../../../
	const FILE_DIR: &str = "../../../tests/protocols/mongodb";
	#[test]
	fn test_mongodb_pcap() -> Result<(), u32> {
		let files = vec![("mongo.pcap", "mongo.result"), ("mongo-msg.pcap", "mongo-msg.result")];
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
		for (header, payload) in packets {
			let Ok(header) = mongodb_header(&payload, payload.len() as u32) else {
				continue;
			};
			output.push_str(&format!("{:?}, {:?}\n", header.message_type(), header));
		}
		Ok(output)
	}
}
