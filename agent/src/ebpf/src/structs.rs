use crate::{
	consts::{IOVLEN_MAX, IOV_MAX, MAX_IOVEC_BUF_SIZE},
	vmlinux::{iovec, mmsghdr},
};
use aya_ebpf::helpers::{
	bpf_probe_read as bpf_helper_read,
	gen::{bpf_probe_read, bpf_probe_read_user},
};
use core::cmp::min;
use mercury_common::{Quintuple, MAX_PAYLOAD_SIZE, MAX_PRE_PAYLOAD_SIZE};

mod mask {
	use mercury_common::MAX_PAYLOAD_SIZE;
	use crate::consts::MAX_IOVEC_BUF_SIZE;
	pub const PAYLOAD_MASK: u32 = MAX_PAYLOAD_SIZE as u32 - 1;
	pub const IOVEC_MASK: u32 = MAX_IOVEC_BUF_SIZE as u32 - 1;
}
pub(crate) struct NormalBuffer {
	buf: u64,
	// TODO: is u64 right here?
	count: u64,
}
impl NormalBuffer {
	pub fn new(buf: u64, count: u64) -> Self {
		Self { buf, count }
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u64) -> Result<u32, u32> {
		let copy_size = min(ret, MAX_PAYLOAD_SIZE);
		let copy_size = min(copy_size, self.count) as u32;
		match unsafe {
			bpf_probe_read(buf as *mut _, copy_size & mask::PAYLOAD_MASK, self.buf as *const u8 as *const _)
		} {
			0 => Ok(copy_size),
			_ => Err(0),
		}
	}
}

/// scatter/gather array
pub(crate) struct VectoredBuffer {
	msg_iov: u64,
	/// elements in msg_iov
	msg_iovlen: u64,
}
impl VectoredBuffer {
	pub fn new(msg_iov: u64, msg_iovlen: u64) -> Self {
		Self { msg_iov, msg_iovlen }
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u64) -> Result<u32, u32> {
		let max = min(ret, MAX_PAYLOAD_SIZE) as u32;

		let round = min(self.msg_iovlen as usize, IOV_MAX);
		let mut offset: u32 = 0;
		let msg_iov = self.msg_iov as *mut iovec;
		for i in 0..round {
			if offset >= max {
				break;
			}

			let iovec = unsafe {
				let iovec_ptr = msg_iov.add(i);
				bpf_helper_read(iovec_ptr)
			}
			.map_err(|_| 0_u32)?;

			let copy_size = min(iovec.iov_len as u32, max - offset) & mask::IOVEC_MASK;
			if unsafe {
				bpf_probe_read_user(
					buf.add((offset & mask::PAYLOAD_MASK) as usize) as *mut _,
					copy_size,
					iovec.iov_base as *const _,
				)
			} != 0
			{
				return Err(0);
			}
			offset += copy_size;
		}
		Ok(offset)
	}
}

pub(crate) struct MsgBuffer {
	mmsg: u64,
	vlen: u64,
}
impl MsgBuffer {
	pub fn new(mmsg: u64, vlen: u64) -> Self {
		Self { mmsg, vlen }
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u64) -> Result<u32, u32> {
		let max = MAX_PAYLOAD_SIZE as u32;
		let round = min(IOV_MAX, self.vlen as usize);
		let mut remain_msg = ret as u32;
		let mut offset: u32 = 0;
		let mmsg = self.mmsg as *mut mmsghdr;
		for i in 0..round {
			if offset >= max || remain_msg == 0 {
				break;
			}
			let mmsghdr = unsafe {
				let mmsghdr_ptr = mmsg.add(i);
				bpf_helper_read(mmsghdr_ptr)
			}
			.map_err(|_| 0_u32)?;

			// TODO: how to handle re-sendmmsg? in which case that msghdr.msg_len is `not` zero
			// let mut inside_offset = mmsghdr.msg_len;
			let msg_hdr = mmsghdr.msg_hdr;
			let msg_iovlen = msg_hdr.msg_iovlen as usize;
			let inside_round = min(remain_msg as usize, min(msg_iovlen, IOVLEN_MAX));
			for j in 0..inside_round {
				if offset >= max {
					break;
				}
				let iovec = unsafe { bpf_helper_read(msg_hdr.msg_iov.add(j)) }.map_err(|_| 0u32)?;
				
				let copy_size = min(iovec.iov_len as u32, max - offset) & mask::IOVEC_MASK;

				let len = unsafe {
					bpf_probe_read(
						buf.add(offset as usize) as *mut _,
						copy_size,
						iovec.iov_base as *const _,
					)
				};
				if len > copy_size as i64 {
					return Err(0);
				}
				offset += copy_size;
			}
			remain_msg -= inside_round as u32;
		}
		Ok(offset)
	}
}
pub(crate) enum Buffer {
	Normal(NormalBuffer),
	Vectored(VectoredBuffer),
	Msg(MsgBuffer),
}

impl Buffer {
	pub fn normal(buffer: u64, count: u64) -> Self {
		Self::Normal(NormalBuffer::new(buffer, count))
	}
	pub fn vectored(msg_iov: u64, msg_iovlen: u64) -> Self {
		Self::Vectored(VectoredBuffer::new(msg_iov, msg_iovlen))
	}
	pub fn msg(msg_buffer: u64, vlen: u64) -> Self {
		Self::Msg(MsgBuffer::new(msg_buffer, vlen))
	}
}
#[repr(C)]
pub(crate) struct Args {
	fd: u64,
	timestamp: u64,
	// quintuple: Quintuple,
	buffer: Buffer,
	seq: u32,
	padding: u32,
}
impl Args {
	pub fn new(fd: u64, seq: u32, timestamp: u64, buffer: Buffer) -> Self {
		Self { fd, seq, buffer, timestamp, padding: 0 }
	}
	pub fn normal(fd: u64, seq: u32, buffer: u64, count: u64, timestamp: u64) -> Self {
		let buffer = Buffer::normal(buffer, count);
		Self::new(fd, seq, timestamp, buffer)
	}
	pub fn vectored(
		fd: u64,
		seq: u32,
		msg_iov: u64,
		msg_iovlen: u64,
		timestamp: u64,
	) -> Self {
		let buffer = Buffer::vectored(msg_iov, msg_iovlen);
		Self::new(fd, seq, timestamp, buffer)
	}
	pub fn msg(fd: u64, seq: u32, msg_buffer: u64, vlen: u64, timestamp: u64) -> Self {
		let buffer = Buffer::msg(msg_buffer, vlen);
		Self::new(fd, seq, timestamp, buffer)
	}
	pub fn fd(&self) -> u64 {
		self.fd
	}
	pub fn seq(&self) -> u32 {
		self.seq
	}
	pub fn timestamp(&self) -> u64 {
		self.timestamp
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u64) -> Result<u32, u32> {
		match &self.buffer {
			Buffer::Normal(normal) => normal.extract(buf, ret),
			Buffer::Vectored(vectored) => vectored.extract(buf, ret),
			Buffer::Msg(msg) => msg.extract(buf, ret),
		}
	}
}

pub(crate) struct PreKey {
	quintuple: Quintuple,
	seq: u32,
}

impl PreKey {
	pub fn new(quintuple: Quintuple, seq: u32) -> Self {
		Self { quintuple, seq }
	}
}
#[derive(Default, Clone)]
pub(crate) struct PrePayload {
	pub buf: [u8; MAX_PRE_PAYLOAD_SIZE as usize],
	pub size: u32,
}

impl PrePayload {
	pub fn new(buf: [u8; MAX_PRE_PAYLOAD_SIZE as usize], size: u32) -> Self {
		Self { buf, size }
	}
}
