use super::{flag::{masks, PacketFlag}, opcode::OpCode, rcode::RCode, DNS};
use nom::{branch::alt, bytes::streaming::tag, combinator::{map, verify}, number::streaming::be_u16, IResult, Parser};

fn flags(i: &[u8]) -> IResult<&[u8], PacketFlag> {
	map(verify(be_u16, |flags: &u16| {
		(flags & masks::RESERVED_MASK) == 0
	}), |flags| PacketFlag::from_bits_truncate(flags)).parse(i)
}

pub(super) fn dns_header(i: &[u8]) -> Result<DNS, u32> {
	let mut header = (be_u16, flags, be_u16, be_u16, be_u16, be_u16);
	let (_, (id, z_flags, questions, answers, name_servers, additional_records)) = header.parse(i).map_err(|_| 0_u32)?;
	Ok(DNS {
	    id,
	    opcode: OpCode::StandardQuery,
	    response_code: RCode::NoError,
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
