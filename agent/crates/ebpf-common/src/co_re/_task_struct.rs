use super::{
	CoRe, file, files_struct,
	generate::{
		self, shim_task_struct_files, shim_task_struct_files_exists, shim_task_struct_se,
		shim_task_struct_se_exists,
	},
	sched_entity,
};
use crate::macros::{__core_read_kernel, kernel_shim};
use aya_ebpf::helpers::r#gen::bpf_get_current_task;

pub type task_struct = CoRe<generate::task_struct>;

impl task_struct {
	kernel_shim!(pub, task_struct, se, sched_entity);
	kernel_shim!(pub, task_struct, files, files_struct);
}

impl task_struct {
	#[inline(always)]
	pub fn current() -> Self {
		Self::from_ptr(unsafe { bpf_get_current_task() } as *const _)
	}

	#[inline(always)]
	/// this is a shortcut function to easily get a file from its fd
	/// looking up the task_struct fdtable.
	pub fn get_fd(&self, fd: usize) -> Option<file> {
		__core_read_kernel!(self, files)?.get_file(fd)
	}
}
