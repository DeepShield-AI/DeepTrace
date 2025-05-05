use super::{MongoDB, OpCode};
use nom::{
	combinator::{map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::le_i32,
	IResult, Parser,
};

fn message_length(expected: u32) -> impl Fn(&[u8]) -> IResult<&[u8], i32> {
	move |i| verify(le_i32, |&message_length| message_length as u32 >= expected).parse(i)
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
	let mut header = (message_length(count), request_id, response_to, opcode);
	let (_, (message_length, request_id, response_to, op_code)) =
		header.parse(i).map_err(|_| 0_u32)?;
	Ok(MongoDB { message_length, request_id, response_to, op_code })
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::tests::load_pcap;
	use trace_common::constants::MAX_PAYLOAD_SIZE;
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
		for (_, payload) in packets {
			let Ok(header) = mongodb_header(&payload, payload.len() as u32) else {
				continue;
			};
			output.push_str(&format!("{:?}, {:?}\n", header.message_type(), header));
		}
		Ok(output)
	}
	#[test]
	fn test_valid() {
		let valid = [
			73, 1, 0, 0, 195, 248, 0, 0, 103, 69, 139, 107, 1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 37, 1, 0, 0, 8, 104, 101, 108, 108, 111, 79, 107, 0,
			1, 8, 105, 115, 109, 97, 115, 116, 101, 114, 0, 1, 3, 116, 111, 112, 111, 108, 111,
			103, 121, 86, 101, 114, 115, 105, 111, 110, 0, 45, 0, 0, 0, 7, 112, 114, 111, 99, 101,
			115, 115, 73, 100, 0, 104, 0, 185, 196, 86, 118, 127, 77, 178, 118, 106, 233, 18, 99,
			111, 117, 110, 116, 101, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 109, 97, 120, 66, 115,
			111, 110, 79, 98, 106, 101, 99, 116, 83, 105, 122, 101, 0, 0, 0, 0, 1, 16, 109, 97,
			120, 77, 101, 115, 115, 97, 103, 101, 83, 105, 122, 101, 66, 121, 116, 101, 115, 0, 0,
			108, 220, 2, 16, 109, 97, 120, 87, 114, 105, 116, 101, 66, 97, 116, 99, 104, 83, 105,
			122, 101, 0, 160, 134, 1, 0, 9, 108, 111, 99, 97, 108, 84, 105, 109, 101, 0, 154, 60,
			61, 94, 150, 1, 0, 0, 16, 108, 111, 103, 105, 99, 97, 108, 83, 101, 115, 115, 105, 111,
			110, 84, 105, 109, 101, 111, 117, 116, 77, 105, 110, 117, 116, 101, 115, 0, 30, 0, 0,
			0, 16, 99, 111, 110, 110, 101, 99, 116, 105, 111, 110, 73, 100, 0, 139, 0, 0, 0, 16,
			109, 105, 110, 87, 105, 114, 101, 86, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, 16,
			109, 97, 120, 87, 105, 114, 101, 86, 101, 114, 115, 105, 111, 110, 0, 13, 0, 0, 0, 8,
			114, 101, 97, 100, 79, 110, 108, 121, 0, 0, 1, 111, 107, 0, 0, 0, 0, 0, 0, 0, 240, 63,
			0,
		];
		let actual = mongodb_header(&valid, valid.len() as u32).unwrap();
		println!("{:?}", actual);
	}
}
