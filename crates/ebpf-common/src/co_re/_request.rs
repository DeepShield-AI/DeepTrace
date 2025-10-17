use super::{
	CoRe, bio,
	generate::{
		self, shim_request___data_len, shim_request___data_len_exists, shim_request_bio,
		shim_request_bio_exists, shim_request_cmd_flags, shim_request_cmd_flags_exists,
		shim_request_io_start_time_ns, shim_request_io_start_time_ns_exists,
		shim_request_start_time_ns, shim_request_start_time_ns_exists,
	},
};
use crate::macros::kernel_shim;

pub type request = CoRe<generate::request>;

impl request {
	kernel_shim!(pub, request, cmd_flags, u32);
	kernel_shim!(pub, request, __data_len, u32);
	kernel_shim!(pub, request, bio, bio);
	kernel_shim!(pub, request, start_time_ns, u64);
	kernel_shim!(pub, request, io_start_time_ns, u64);
}
