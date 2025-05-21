use super::{
	constants::{COMPACT_PROTOCOL_ID, COMPACT_PROTOCOL_VERSION},
	Kind, Thrift,
};
use nom::{
	bytes::streaming::tag,
	combinator::{map, map_res, verify},
	error::{Error, ErrorKind},
	number::streaming::be_u8,
	IResult, Parser,
};

fn protocol_id(i: &[u8]) -> IResult<&[u8], u8> {
	map(tag([COMPACT_PROTOCOL_ID].as_slice()), |magic: &[u8]| magic[0]).parse(i)
}

fn kind_and_version(i: &[u8]) -> IResult<&[u8], Kind> {
	map_res(
		verify(be_u8, |kind_and_version| kind_and_version & 0b1_1111 == COMPACT_PROTOCOL_VERSION),
		|kind_and_version: u8| {
			Kind::try_from(kind_and_version >> 5).map_err(|_| Error::new(i, ErrorKind::MapRes))
		},
	)
	.parse(i)
}

#[inline]
pub(crate) fn thrift_compact_header(i: &[u8]) -> Result<Thrift, u32> {
	let mut compact = (protocol_id, kind_and_version);

	let (_, (_, kind)) = compact.parse(i).map_err(|_| 0_u32)?;

	Ok(Thrift { kind })
}
