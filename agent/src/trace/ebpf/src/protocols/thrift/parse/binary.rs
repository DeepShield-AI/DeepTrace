use super::{constants::BINARY_PROTOCOL_VERSION, Kind, Thrift};
use nom::{
	branch::alt,
	combinator::{map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::{be_i32, be_u16, be_u32, be_u8},
	sequence::preceded,
	IResult, Parser,
};

fn version(i: &[u8]) -> IResult<&[u8], u16> {
	verify(be_u16, |version| version >> 15 == 1 && version & 0x7FFF == BINARY_PROTOCOL_VERSION)
		.parse(i)
}

fn kind(i: &[u8]) -> IResult<&[u8], Kind> {
	map_res(be_u8, |kind: u8| {
		Kind::try_from(kind & 0b111).map_err(|_| Error::new(i, ErrorKind::MapRes))
	})
	.parse(i)
}

fn name_length(i: &[u8]) -> IResult<&[u8], i32> {
	verify(be_i32, |&size| size >= 0).parse(i)
}

#[inline]
pub(crate) fn thrift_binary_header(i: &[u8], len: u32) -> Result<Thrift, u32> {
	let mut binary = alt((
		(version, be_u8, kind, name_length),
		preceded(verify(be_u32, |&length| length == len - 4), (version, be_u8, kind, name_length)),
	));
	let (_, (_, _, kind, _)) = binary.parse(i).map_err(|_| 0_u32)?;
	Ok(Thrift { kind })
}
#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::tests::load_pcap;
	use trace_common::constants::MAX_PAYLOAD_SIZE;
	const FILE_DIR: &str = "../../../tests/protocols/thrift";
	#[test]
	fn test_binary_thrift_pcap() -> Result<(), u32> {
		let files = vec![("social.pcap", "social.result"), ("tt.pcap", "tt.result")];
		for (actual, expected) in files {
			let actual = format!("{}/{}", FILE_DIR, actual);
			let expected = format!("{}/{}", FILE_DIR, expected);
			let expected = std::fs::read_to_string(&expected).map_err(|_| 0_u32)?;
			let actual = run(&actual).map_err(|_| 0_u32)?;
			assert_eq!(actual, expected, "{} != {}", actual, expected);
		}
		Ok(())
	}

	fn run(actually: &str) -> Result<String, u32> {
		let packets = load_pcap(actually, MAX_PAYLOAD_SIZE as usize)?;
		if packets.is_empty() {
			return Err(0);
		}
		let mut output = String::new();
		for (_, payload) in packets {
			let Ok(header) = thrift_binary_header(&payload, payload.len() as u32) else {
				continue;
			};
			output.push_str(&format!("{:?}, {:?}\n", header.message_type(), header));
		}
		Ok(output)
	}
}
