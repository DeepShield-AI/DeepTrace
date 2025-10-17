#![allow(non_camel_case_types)]

pub use _bio::*;
pub use _blkcg::*;
pub use _blkcg_gq::*;
pub use _block_device::*;
pub use _cfs_rq::*;
pub use _cgroup::*;
pub use _cgroup_subsys_state::*;
pub use _fdtable::*;
pub use _file::*;
pub use _files_struct::*;
pub use _gendisk::*;
pub use _inet_connection_sock::*;
pub use _inet_sock::*;
pub use _iovec::*;
pub use _kernfs_node::*;
pub use _mmsghdr::*;
pub use _request::*;
pub use _rq::*;
pub use _sched_entity::*;
pub use _sock::*;
pub use _socket::*;
pub use _socket_common::*;
pub use _task_struct::*;
pub use _tcp_sock::*;
pub use _user_msghdr::*;

mod _bio;
mod _blkcg;
mod _blkcg_gq;
mod _block_device;
mod _cfs_rq;
mod _cgroup;
mod _cgroup_subsys_state;
mod _fdtable;
mod _file;
mod _files_struct;
mod _gendisk;
mod _inet_connection_sock;
mod _inet_sock;
mod _iovec;
mod _kernfs_node;
mod _mmsghdr;
mod _request;
mod _rq;
mod _sched_entity;
mod _sock;
mod _socket;
mod _socket_common;
mod _task_struct;
mod _tcp_sock;
mod _user_msghdr;
#[allow(dead_code)]
mod generate;

#[derive(Clone, Copy)]
pub struct CoRe<P> {
	ptr: *const P,
}

impl<P> PartialEq for CoRe<P> {
	fn eq(&self, other: &Self) -> bool {
		self.ptr == other.ptr
	}
}

impl<P> From<*mut P> for CoRe<P> {
	fn from(value: *mut P) -> Self {
		Self::from_ptr(value)
	}
}

impl<P> From<*const P> for CoRe<P> {
	fn from(value: *const P) -> Self {
		Self::from_ptr(value)
	}
}

impl<P> CoRe<P> {
	#[inline(always)]
	pub fn bpf_read(&self) -> Result<*const P, i64> {
		unsafe { aya_ebpf::helpers::bpf_probe_read(&self.ptr) }
	}

	#[inline(always)]
	pub const fn is_null(&self) -> bool {
		self.ptr.is_null()
	}

	pub const fn as_ptr(&self) -> *const P {
		self.ptr as *mut _
	}

	const fn as_ptr_mut(&self) -> *mut P {
		self.ptr as *mut _
	}

	pub const fn from_ptr(ptr: *const P) -> Self {
		CoRe { ptr: ptr as *const _ }
	}
}
