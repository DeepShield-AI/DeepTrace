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

pub(crate) struct NormalBuffer {
	buf: *const u8,
	// TODO: is u32 right here?
	count: u32,
}
impl NormalBuffer {
	pub fn new(buf: *const u8, count: u32) -> Self {
		Self { buf, count }
	}
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let copy_size = min(ret, MAX_PAYLOAD_SIZE);
		let copy_size = min(copy_size, self.count);
		match unsafe {
			bpf_probe_read(buf as *mut _, copy_size & (MAX_PAYLOAD_SIZE - 1), self.buf as *const _)
		} {
			0 => Ok(copy_size),
			_ => Err(0),
		}
	}
}

/// scatter/gather array
pub(crate) struct VectoredBuffer {
	msg_iov: *mut iovec,
	/// elements in msg_iov
	msg_iovlen: u64,
}
impl VectoredBuffer {
	pub fn new(msg_iov: *mut iovec, msg_iovlen: u64) -> Self {
		Self { msg_iov, msg_iovlen }
	}
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let max = min(ret, MAX_PAYLOAD_SIZE);

		let round = min(self.msg_iovlen as usize, IOV_MAX);
		let mut offset: u32 = 0;
		for i in 0..round {
			if offset >= max {
				break;
			}

			let iovec = unsafe {
				let iovec_ptr = self.msg_iov.add(i);
				bpf_helper_read(iovec_ptr)
			}
			.map_err(|_| 0_u32)?;

			let mut iov_len = iovec.iov_len as u32;
			if 0 < iov_len && iov_len < MAX_IOVEC_BUF_SIZE {
				iov_len &= MAX_IOVEC_BUF_SIZE - 1;
			} else if iov_len >= MAX_IOVEC_BUF_SIZE {
				iov_len = MAX_IOVEC_BUF_SIZE;
			} else {
				return Err(0);
			}

			let copy_size = min(iov_len, max.saturating_sub(offset));
			if unsafe {
				bpf_probe_read_user(
					buf.add(offset as usize) as *mut _,
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
	mmsg: *mut mmsghdr,
	vlen: u32,
}
impl MsgBuffer {
	pub fn new(mmsg: *mut mmsghdr, vlen: u32) -> Self {
		Self { mmsg, vlen }
	}
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let max = MAX_PAYLOAD_SIZE;
		let round = min(IOV_MAX, self.vlen as usize);
		let mut remain_msg = ret;
		let mut offset: u32 = 0;
		for i in 0..round {
			if offset >= max || remain_msg == 0 {
				break;
			}
			let mmsghdr = unsafe {
				let mmsghdr_ptr = self.mmsg.add(i);
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
				let mut iov_len = iovec.iov_len as u32;

				if 0 < iov_len && iov_len < MAX_IOVEC_BUF_SIZE {
					iov_len &= MAX_IOVEC_BUF_SIZE - 1;
				} else if iov_len >= MAX_IOVEC_BUF_SIZE {
					iov_len = MAX_IOVEC_BUF_SIZE;
				} else {
					return Err(0);
				}

				let copy_size = if offset + iov_len > max { max - offset } else { iov_len };

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
	pub fn normal(buffer: *const u8, count: u32) -> Self {
		Self::Normal(NormalBuffer::new(buffer, count))
	}
	pub fn vectored(msg_iov: *mut iovec, msg_iovlen: u64) -> Self {
		Self::Vectored(VectoredBuffer::new(msg_iov, msg_iovlen))
	}
	pub fn msg(msg_buffer: *mut mmsghdr, vlen: u32) -> Self {
		Self::Msg(MsgBuffer::new(msg_buffer, vlen))
	}
}

pub(crate) struct Args {
	// pub tgid: u32,
	fd: u32,
	seq: u32,
	timestamp: u64,
	// quintuple: Quintuple,
	buffer: Buffer,
}
impl Args {
	pub fn new(fd: u32, seq: u32, timestamp: u64, buffer: Buffer) -> Self {
		Self { fd, seq, buffer, timestamp }
	}
	pub fn normal(fd: u32, seq: u32, buffer: *const u8, count: u32, timestamp: u64) -> Self {
		let buffer = Buffer::normal(buffer, count);
		Self::new(fd, seq, timestamp, buffer)
	}
	pub fn vectored(
		fd: u32,
		seq: u32,
		msg_iov: *mut iovec,
		msg_iovlen: u64,
		timestamp: u64,
	) -> Self {
		let buffer = Buffer::vectored(msg_iov, msg_iovlen);
		Self::new(fd, seq, timestamp, buffer)
	}
	pub fn msg(fd: u32, seq: u32, msg_buffer: *mut mmsghdr, vlen: u32, timestamp: u64) -> Self {
		let buffer = Buffer::msg(msg_buffer, vlen);
		Self::new(fd, seq, timestamp, buffer)
	}
	pub fn fd(&self) -> u32 {
		self.fd
	}
	pub fn seq(&self) -> u32 {
		self.seq
	}
	pub fn timestamp(&self) -> u64 {
		self.timestamp
	}
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
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
