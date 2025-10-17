use super::{
	CoRe, fdtable, file,
	generate::{self, shim_files_struct_fdt, shim_files_struct_fdt_exists},
};
use crate::macros::{__core_read_kernel, kernel_shim};
use aya_ebpf::helpers::bpf_probe_read_kernel;

pub type files_struct = CoRe<generate::files_struct>;

impl files_struct {
	kernel_shim!(pub, files_struct, fdt, fdtable);
}

impl files_struct {
	/// gets a file corresponding to a file descriptor. We lookup in fdtable
	/// as it always points to the good array containing fds.
	/// NB: fd_array is not reliable because it can be remapped.
	#[inline(always)]
	pub fn get_file(&self, fd: usize) -> Option<file> {
		if fd <= __core_read_kernel!(self, fdt, max_fds)? as usize {
			let ptr =
				unsafe { bpf_probe_read_kernel(__core_read_kernel!(self, fdt, fd)?.add(fd)).ok() }?;
			return Some(ptr.into());
		}
		None
	}
}
