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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::tests::load_pcap;
	use mercury_common::consts::MAX_PAYLOAD_SIZE;

	const FILE_DIR: &str = "../../../tests/protocols/dns";
	// #[test]
	// #[ignore]
	// fn test_dns_pcap() -> Result<(), u32> {
	//     let files = vec![
	//         ("redis.pcap", "redis.result"),
	//         ("redis-error.pcap", "redis-error.result"),
	//         ("redis-debug.pcap", "redis-debug.result"),
	//     ];
	//     for (actual, expected) in files {
	//         let actual = format!("{}/{}", FILE_DIR, actual);
	//         let expected = format!("{}/{}", FILE_DIR, expected);
	//         let actual = run(&actual).map_err(|_| 0_u32)?;
	//         let expected = std::fs::read_to_string(expected).map_err(|_| 0_u32)?;
	//         assert_eq!(actual, expected, "{} != {}", actual, expected);
	//     }
	//     Ok(())
	// }

	// fn run(actual: &str) -> Result<String, u32> {
	//     let packets = load_pcap(actual, MAX_PAYLOAD_SIZE as usize)?;
	//     if packets.is_empty() {
	//         return Err(0);
	//     }
	//     let mut output = String::new();
	//     for (header, payload) in packets {
	//         let Ok(header) = dns_header(&payload) else {
	//             continue;
	//         };
	//         output.push_str(&format!("{:?}, {}\n", header.message_type(), header));
	//     }
	//     Ok(output)
	// }
}
