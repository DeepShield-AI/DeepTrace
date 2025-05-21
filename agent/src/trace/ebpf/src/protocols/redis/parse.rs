use super::Redis;
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
// ref: <https://redis.io/docs/latest/develop/reference/protocol-spec/#simple-errors>
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

// TODO: need more strict check
pub(super) fn redis(i: &[u8], count: u32) -> Result<Redis, u32> {
	let (i, first) = prefix(i).map_err(|_| 0_u32)?;
	let mut redis = Redis::new();
	redis.first = first;
	if first == b'-' {
		error(i).map_err(|_| 0_u32)?;
		Ok(redis)
	} else {
		let mut p = count;
		let mut found = false;
		for r in 0..5 + 2 {
			if i[r] == b'\r' && i[r + 1] == b'\n' {
				p = r as u32 + 2;
				found = true;
				break;
			}
		}
		if !found || p > count {
			return Err(0_u32);
		}

		if first != b'*' || p == count {
			return Ok(redis);
		}
		found = false;
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
				found = true;
				break;
			}
		}
		if !found {
			return Err(0_u32);
		}
		Ok(redis)
	}
}
