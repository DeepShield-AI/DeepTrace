use super::{
	CoRe,
	generate::{
		self, shim_user_msghdr_msg_iov, shim_user_msghdr_msg_iov_exists,
		shim_user_msghdr_msg_iovlen, shim_user_msghdr_msg_iovlen_exists,
	},
	iovec,
};
use crate::macros::kernel_shim;

pub type user_msghdr = CoRe<generate::user_msghdr>;

impl user_msghdr {
	kernel_shim!(pub, user_msghdr, msg_iov, iovec);
	kernel_shim!(pub, user_msghdr, msg_iovlen, u64);
}
