use core::mem;

/// An integer byte that distinguishes the actual message.
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
#[repr(u8)]
pub(super) enum OpCode {
	ERROR = 0x00,
	STARTUP = 0x01,
	READY = 0x02,
	AUTHENTICATE = 0x03,
	OPTIONS = 0x05,
	SUPPORTED = 0x06,
	QUERY = 0x07,
	RESULT = 0x08,
	PREPARE = 0x09,
	EXECUTE = 0x0A,
	REGISTER = 0x0B,
	EVENT = 0x0C,
	BATCH = 0x0D,
	AUTH_CHALLENGE = 0x0E,
	AUTH_RESPONSE = 0x0F,
	AUTH_SUCCESS = 0x10,
}

impl TryFrom<u8> for OpCode {
	type Error = u8;

	#[inline(always)]
	fn try_from(opcode: u8) -> Result<Self, Self::Error> {
		if (0..=3).contains(&opcode) || (5..=16).contains(&opcode) {
			return Ok(from_u8(opcode));
		}
		Err(opcode)
	}
}

#[inline(always)]
fn from_u8(x: u8) -> OpCode {
	unsafe { mem::transmute(x) }
}
