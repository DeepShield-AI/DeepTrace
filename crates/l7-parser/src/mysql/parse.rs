use super::{Com, MySQL};
use aya_ebpf::helpers::bpf_get_current_comm;
use ebpf_common::error::{Result, code::*};
use nom::{
	IResult, Parser,
	combinator::{map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::{le_u8, le_u24},
};
use observ_trace_common::{Direction, MessageType};
fn payload_length(expected: usize) -> impl Fn(&[u8]) -> IResult<&[u8], u32> {
	move |i| verify(le_u24, |&payload_length| payload_length as usize + 4 == expected).parse(i)
}
fn com(i: &[u8]) -> IResult<&[u8], Com> {
	map_res(le_u8, |com: u8| Com::try_from(com).map_err(|_| Error::new(i, ErrorKind::MapRes)))
		.parse(i)
}

pub(super) fn mysql(i: &[u8], count: usize, direction: Direction) -> Result<MySQL> {
	let mut header = (payload_length(count), le_u8, com);
	let (_, (payload_length, _sequence_id, _)) = header.parse(i).map_err(|_| PARSE_MYSQL_FAILED)?;

	if count < 5 || payload_length == 0 {
		return Err(MYSQL_PAYLOAD_LENGTH_INVALID);
	}

	let mysqld = bpf_get_current_comm().is_ok_and(|comm| &comm[..4] == b"mysqld\0");
	let type_ =
		if mysqld && direction == Direction::Ingress || !mysqld && direction == Direction::Egress {
			MessageType::Request
		} else {
			MessageType::Response
		};
	Ok(MySQL { type_ })
}
