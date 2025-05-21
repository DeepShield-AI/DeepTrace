use super::HTTP1;
use nom::{branch::alt, bytes::streaming::tag, IResult, Parser};
use trace_common::message::MessageType;

fn http1_method(i: &[u8]) -> IResult<&[u8], &[u8]> {
	alt((
		tag("GET ".as_bytes()),
		tag("POST ".as_bytes()),
		tag("PUT ".as_bytes()),
		tag("DELETE ".as_bytes()),
		tag("HEAD ".as_bytes()),
		tag("OPTIONS ".as_bytes()),
		tag("PATCH ".as_bytes()),
	))
	.parse(i)
}

fn http1_response(i: &[u8]) -> IResult<&[u8], &[u8]> {
	alt((tag("HTTP/1.0 ".as_bytes()), tag("HTTP/1.1 ".as_bytes()))).parse(i)
}

pub(super) fn http1(i: &[u8]) -> Result<HTTP1, u32> {
	if http1_method(i).is_ok() {
		return Ok(HTTP1 { type_: MessageType::Request });
	} else if http1_response(i).is_ok() {
		return Ok(HTTP1 { type_: MessageType::Response });
	}
	Err(0)
}
