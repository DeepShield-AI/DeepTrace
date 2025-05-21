use super::{Com, MySQL};
use aya_ebpf::helpers::bpf_get_current_comm;
use nom::{
	combinator::{map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::{le_u24, le_u8},
	IResult, Parser,
};
use trace_common::{message::MessageType, structs::Direction};
fn payload_length(expected: u32) -> impl Fn(&[u8]) -> IResult<&[u8], u32> {
	move |i| verify(le_u24, |&payload_length| payload_length + 4 == expected).parse(i)
}
fn com(i: &[u8]) -> IResult<&[u8], Com> {
	map_res(le_u8, |com: u8| Com::try_from(com).map_err(|_| Error::new(i, ErrorKind::MapRes)))
		.parse(i)
}

pub(super) fn mysql_header(i: &[u8], count: u32, direction: Direction) -> Result<MySQL, u32> {
	let mut header = (payload_length(count), le_u8, com);
	let (_, (payload_length, _sequence_id, _)) = header.parse(i).map_err(|_| 0_u32)?;

	if count < 5 || payload_length == 0 {
		return Err(0_u32);
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
