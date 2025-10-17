use core::mem;

#[repr(u8)]
#[derive(Debug)]
pub enum Com {
	Query = 0x03,
	Connect = 0x0b,
	StmtPrepare = 0x16,
	StmtExecute = 0x17,
	StmtClose = 0x19,
	StmtQuit = 0x01,
}

impl TryFrom<u8> for Com {
	type Error = ();

	#[inline(always)]
	fn try_from(opcode: u8) -> Result<Self, Self::Error> {
		if opcode == 0x01 ||
			opcode == 0x03 ||
			opcode == 0x0b ||
			opcode == 0x16 ||
			opcode == 0x17 ||
			opcode == 0x19
		{
			return Ok(from_u8(opcode));
		}
		Err(())
	}
}

#[inline(always)]
fn from_u8(x: u8) -> Com {
	unsafe { mem::transmute(x) }
}
