use super::{
	CoRe,
	generate::{
		self, shim_iovec_iov_base, shim_iovec_iov_base_exists, shim_iovec_iov_len,
		shim_iovec_iov_len_exists,
	},
};
use crate::macros::kernel_shim;
use aya_ebpf::cty::c_void;

pub type iovec = CoRe<generate::iovec>;

impl iovec {
	kernel_shim!(pub, iovec, iov_base, *mut c_void);
	kernel_shim!(pub, iovec, iov_len, u64);
}

impl iovec {
	#[inline(always)]
	pub fn get(&self, index: usize) -> Self {
		unsafe { self.as_ptr().add(index).into() }
	}
}
