use core::mem;

/// Possible OPCODE values for a DNS packet, use to specify the type of operation.  
/// [RFC 1035](https://tools.ietf.org/html/rfc1035): A four bit field that specifies kind of query in this message.  
/// This value is set by the originator of a query and copied into the response.
#[repr(u16)]
#[derive(PartialEq)]
pub enum OpCode {
	/// Normal query
	StandardQuery = 0,
	/// Inverse query (query a name by IP)
	InverseQuery = 1,
	/// Server status request
	ServerStatusRequest = 2,
	/// Notify query
	Notify = 4,
	/// Update query [RFC 2136](https://datatracker.ietf.org/doc/html/rfc2136)
	Update = 5,
	/// Reserved opcode for future use
	Reserved,
}

impl TryFrom<u16> for OpCode {
	type Error = u16;

	#[inline(always)]
	fn try_from(opcode: u16) -> Result<Self, Self::Error> {
		if (0..=2).contains(&opcode) || (4..=5).contains(&opcode) {
			return Ok(from_u16(opcode));
		}
		Err(opcode)
	}
}

#[inline(always)]
fn from_u16(x: u16) -> OpCode {
	unsafe { mem::transmute(x) }
}
