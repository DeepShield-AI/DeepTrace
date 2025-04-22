use core::mem;

#[repr(u8)]
#[derive(Debug)]
pub(crate) enum Kind {
	Call = 1,
	Reply = 2,
	Exception = 3,
	Oneway = 4,
}

impl TryFrom<u8> for Kind {
	type Error = ();

	fn try_from(kind: u8) -> Result<Self, Self::Error> {
		if (1..=4).contains(&kind) {
			return Ok(from_u8(kind));
		}
		Err(())
	}
}

#[inline(always)]
fn from_u8(x: u8) -> Kind {
	unsafe { mem::transmute(x) }
}
