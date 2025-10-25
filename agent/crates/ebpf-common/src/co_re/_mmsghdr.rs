use super::{
	CoRe,
	generate::{
		self, shim_mmsghdr_msg_hdr, shim_mmsghdr_msg_hdr_exists, shim_mmsghdr_msg_len,
		shim_mmsghdr_msg_len_exists,
	},
	user_msghdr,
};
use crate::macros::kernel_shim;

pub type mmsghdr = CoRe<generate::mmsghdr>;

impl mmsghdr {
	kernel_shim!(pub, mmsghdr, msg_hdr, user_msghdr);
	kernel_shim!(pub, mmsghdr, msg_len, u32);
}

impl mmsghdr {
	#[inline(always)]
	pub fn get(&self, index: usize) -> Self {
		unsafe { self.as_ptr().add(index).into() }
	}
}
