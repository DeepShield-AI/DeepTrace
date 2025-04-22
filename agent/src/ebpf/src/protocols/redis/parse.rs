use super::Redis;
use mercury_common::mask::INFER_MASK;
use nom::{branch::alt, bytes::streaming::tag, combinator::map, IResult, Parser};
fn prefix(i: &[u8]) -> IResult<&[u8], u8> {
	map(
		alt((
			tag("+".as_bytes()),
			tag("-".as_bytes()),
			tag(":".as_bytes()),
			tag("$".as_bytes()),
			tag("*".as_bytes()),
			tag("_".as_bytes()),
			tag("#".as_bytes()),
			tag(",".as_bytes()),
			tag("(".as_bytes()),
			tag("!".as_bytes()),
			tag("=".as_bytes()),
			tag("%".as_bytes()),
			tag("~".as_bytes()),
			tag(">".as_bytes()),
		)),
		|first: &[u8]| first[0],
	)
	.parse(i)
}
// TODO: add all error message check
fn error(i: &[u8]) -> IResult<&[u8], &[u8]> {
	alt((
		tag("ERR".as_bytes()),
		tag("WRONGTYPE".as_bytes()),
		tag("Invalid".as_bytes()),
		tag("NOAUTH".as_bytes()),
	))
	.parse(i)
}

// fn check_cslf(i: &[u8]) -> IResult<&[u8], &[u8]> {
// 	terminated(take_until("\r\n".as_bytes()), take(2_usize)).parse(i)
// }

fn is_contain_crlf(i: &[u8], count: u32) -> bool {
	for r in 0..((count & INFER_MASK) as usize) {
		if i[r] == b'\r' && i[r + 1] == b'\n' {
			return true;
		}
	}
	false
}

pub(super) fn redis(i: &[u8], count: u32) -> Result<Redis, u32> {
	let (i, first) = prefix(i).map_err(|_| 0_u32)?;
	if first as char == '-' {
		error(i).map_err(|_| 0_u32)?;
	} else if !is_contain_crlf(i, count - 1) {
		return Err(0_u32);
	}
	Ok(Redis { first })
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::utils::tests::load_pcap;
	use mercury_common::consts::MAX_PAYLOAD_SIZE;

	const FILE_DIR: &str = "../../../tests/protocols/redis";
	#[test]
	fn test_redis_pcap() -> Result<(), u32> {
		let files = vec![
			("redis.pcap", "redis.result"),
			("redis-error.pcap", "redis-error.result"),
			("redis-debug.pcap", "redis-debug.result"),
		];
		for (actual, expected) in files {
			let actual = format!("{}/{}", FILE_DIR, actual);
			let expected = format!("{}/{}", FILE_DIR, expected);
			let actual = run(&actual).map_err(|_| 0_u32)?;
			let expected = std::fs::read_to_string(expected).map_err(|_| 0_u32)?;
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
			let Ok(header) = redis(&payload, payload.len() as u32) else {
				continue;
			};
			output.push_str(&format!("{:?}, {}\n", header.message_type(), header));
		}
		Ok(output)
	}
}
