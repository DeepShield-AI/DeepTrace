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

// fn command(i: &[u8]) -> IResult<&[u8], &[u8]> {
// 	alt([
// 		tag("A".as_bytes()),
// 		tag("B".as_bytes()),
// 		tag("C".as_bytes()),
// 		tag("D".as_bytes()),
// 		tag("E".as_bytes()),
// 		tag("F".as_bytes()),
// 		tag("GE".as_bytes()),
// 		tag("H".as_bytes()),
// 		tag("I".as_bytes()),
// 		tag("JS".as_bytes()),
// 		tag("KE".as_bytes()),
// 		tag("L".as_bytes()),
// 		tag("M".as_bytes()),
// 		tag("OB".as_bytes()),
// 		tag("P".as_bytes()),
// 		tag("QU".as_bytes()),
// 		tag("R".as_bytes()),
// 		tag("S".as_bytes()),
// 		tag("T".as_bytes()),
// 		tag("UN".as_bytes()),
// 		tag("WA".as_bytes()),
// 		tag("X".as_bytes()),
// 		tag("Z".as_bytes()),
// 	])
// 	.parse(i)
// }

fn is_contain_crlf(i: &[u8], count: u32) -> bool {
	for r in 0..((count & INFER_MASK) as usize) {
		if i[r] == b'\r' && i[r + 1] == b'\n' {
			return true;
		}
	}
	false
}
// TODO: need more strict check
pub(super) fn redis(i: &[u8], count: u32) -> Result<Redis, u32> {
	let (i, first) = prefix(i).map_err(|_| 0_u32)?;
	let mut redis = Redis::new();
	redis.first = first;
	if first == b'-' {
		error(i).map_err(|_| 0_u32)?;
		return Ok(redis)
	} else {
		let mut p = count;
		for r in 0..3 + 2 {
			if i[r] == b'\r' && i[r + 1] == b'\n' {
				p = r as u32 + 2;
				break;
			}
		}
		if p > count {
			return Err(0_u32);
		}
		if first != b'*' || p == count {
			return Ok(redis);
		}

		for r in 5..5 + 2 + 5 + 1 {
			if i[r] == b'\r' && i[r + 1] == b'\n' {
				let c = i[r + 2] as char;
				let s = i[r + 3] as char;
				match c {
					'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' |
					'M' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'W' | 'X' | 'Z'
						if s.is_ascii_uppercase() =>
					{
						redis.is_command = true;
					},
					_ => {},
				}
				break;
			}
		}
		return Ok(redis);
	}
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
		for (_, payload) in packets {
			let Ok(header) = redis(&payload, payload.len() as u32) else {
				continue;
			};
			output.push_str(&format!("{:?}, {}\n", header.message_type(), header));
		}
		Ok(output)
	}
}
