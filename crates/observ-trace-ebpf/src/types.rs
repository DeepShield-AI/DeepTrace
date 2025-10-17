use core::cmp::min;
use ebpf_common::{
	buffer::Buffer,
	co_re::{iovec, mmsghdr},
	constants::{IOV_MAX, IOVLEN_MAX},
	error::Result,
};
use l7_parser::constants::MAX_INFER_PAYLOAD_SIZE;
use observ_trace_common::{protocols::L7Protocol, structs::Direction};

#[repr(C)]
pub(crate) struct Args {
	pub fd: u64,
	pub enter_time: u64,
	// quintuple: Quintuple,
	pub buffer: SysBufPtr,
	pub enter_seq: u32,
	pub padding: u32,
}
impl Args {
	#[inline(always)]
	pub fn new(fd: u64, timestamp: u64, buffer: SysBufPtr, enter_seq: u32) -> Self {
		Self { fd, buffer, enter_time: timestamp, enter_seq, padding: 0 }
	}
	#[inline(always)]
	pub fn from_ubuf(fd: u64, buf: *mut u8, count: u32, timestamp: u64, enter_seq: u32) -> Self {
		Self::new(fd, timestamp, SysBufPtr::Ubuf(buf, count), enter_seq)
	}
	#[inline(always)]
	pub fn from_msg(fd: u64, vec: iovec, vlen: u32, timestamp: u64, enter_seq: u32) -> Self {
		Self::new(fd, timestamp, SysBufPtr::Msg(vec, vlen), enter_seq)
	}
	#[inline(always)]
	pub fn from_mmsg(fd: u64, mmsg: mmsghdr, vlen: u32, timestamp: u64, enter_seq: u32) -> Self {
		Self::new(fd, timestamp, SysBufPtr::MMsg(mmsg, vlen), enter_seq)
	}
	#[inline(always)]
	pub fn extract<const N: usize>(&self, mut buffer: Buffer<N>, ret: u32) -> Result<()> {
		match self.buffer {
			SysBufPtr::Ubuf(ubuf, size) => buffer.read_user_at(ubuf, min(size, ret)),
			SysBufPtr::Msg(iovec, vlen) =>
				buffer.fill_from_iovec::<IOV_MAX>(iovec, vlen, Some(ret as usize)),
			SysBufPtr::MMsg(mmsg, vlen) =>
				buffer.fill_from_mmsghdr::<IOVLEN_MAX>(mmsg, min(vlen, ret), None),
		}
	}
}

/// syscall buffer type
pub(crate) enum SysBufPtr {
	Ubuf(*mut u8, u32),
	Msg(iovec, u32),
	MMsg(mmsghdr, u32),
}

#[repr(C)]
pub struct SocketInfo {
	pub uuid: u32,
	pub exit_seq: u32,
	pub seq: u32,
	pub direction: Direction,
	pub pre_direction: Direction,
	pub l7protocol: L7Protocol,
	padding: u8,
	pub prev_buf: Buffer<MAX_INFER_PAYLOAD_SIZE>,
}
