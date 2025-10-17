use super::{
	CoRe,
	generate::{
		self, shim_fdtable_fd, shim_fdtable_fd_exists, shim_fdtable_max_fds,
		shim_fdtable_max_fds_exists,
	},
};
use crate::macros::kernel_shim;

pub type fdtable = CoRe<generate::fdtable>;

impl fdtable {
	kernel_shim!(pub, fdtable, max_fds, u32);
	kernel_shim!(pub, fdtable, fd, *mut *mut generate::file);
}
