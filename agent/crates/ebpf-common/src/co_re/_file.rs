use super::{
	CoRe,
	generate::{self, shim_file_private_data, shim_file_private_data_exists},
};
use crate::macros::kernel_shim;
use aya_ebpf::cty::c_void;

pub type file = CoRe<generate::file>;

impl file {
	kernel_shim!(pub, file, private_data, *mut c_void);
}
