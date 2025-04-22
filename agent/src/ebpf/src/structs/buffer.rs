use crate::vmlinux::{iovec, mmsghdr};
use aya_ebpf::helpers::gen::bpf_probe_read;
use core::{cmp::min, mem::MaybeUninit};
use mercury_common::{
	consts::{
		IOVLEN_MAX, IOV_MAX, MAX_INFER_PAYLOAD_SIZE, MAX_IOVEC_PAYLOAD_SIZE, MAX_PAYLOAD_SIZE,
	},
	mask::{INFER_MASK, IOVEC_MASK, PAYLOAD_MASK},
	structs::Direction,
};

#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub(crate) struct NormalBuffer {
	buf: u64,
	count: u64,
}
impl NormalBuffer {
	#[inline]
	pub fn new(buf: u64, count: u64) -> Self {
		Self { buf, count }
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let copy_size = min(min(ret, MAX_PAYLOAD_SIZE), self.count as u32);
		if copy_size >= MAX_PAYLOAD_SIZE {
			match unsafe {
				bpf_probe_read(buf as *mut _, PAYLOAD_MASK, self.buf as *const u8 as *const _)
			} {
				0 => Ok(PAYLOAD_MASK),
				_ => Err(0),
			}
		} else {
			match unsafe {
				bpf_probe_read(
					buf as *mut _,
					copy_size & PAYLOAD_MASK,
					self.buf as *const u8 as *const _,
				)
			} {
				0 => Ok(copy_size & PAYLOAD_MASK),
				_ => Err(0),
			}
		}
	}
	#[inline]
	pub fn infer_extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let copy_size = min(min(ret, MAX_INFER_PAYLOAD_SIZE), self.count as u32);
		if copy_size >= MAX_INFER_PAYLOAD_SIZE {
			match unsafe {
				bpf_probe_read(
					buf as *mut _,
					MAX_INFER_PAYLOAD_SIZE,
					self.buf as *const u8 as *const _,
				)
			} {
				0 => Ok(MAX_INFER_PAYLOAD_SIZE),
				_ => Err(0),
			}
		} else {
			match unsafe {
				bpf_probe_read(
					buf as *mut _,
					copy_size & INFER_MASK,
					self.buf as *const u8 as *const _,
				)
			} {
				0 => Ok(copy_size & INFER_MASK),
				_ => Err(0),
			}
		}
	}
	#[inline]
	pub fn save_prev(&self, buf: *mut u8) -> Result<u32, u32> {
		let copy_size = min(MAX_INFER_PAYLOAD_SIZE, self.count as u32);
		if copy_size >= MAX_INFER_PAYLOAD_SIZE {
			match unsafe {
				bpf_probe_read(
					buf as *mut _,
					MAX_INFER_PAYLOAD_SIZE,
					self.buf as *const u8 as *const _,
				)
			} {
				0 => Ok(MAX_INFER_PAYLOAD_SIZE),
				_ => Err(0),
			}
		} else {
			match unsafe {
				bpf_probe_read(
					buf as *mut _,
					copy_size & INFER_MASK,
					self.buf as *const u8 as *const _,
				)
			} {
				0 => Ok(copy_size & INFER_MASK),
				_ => Err(0),
			}
		}
	}
}

/// scatter/gather array
#[derive(Copy, Clone)]
pub(crate) struct VectoredBuffer {
	msg_iov: u64,
	/// elements in msg_iov
	msg_iovlen: u64,
}
impl VectoredBuffer {
	#[inline]
	pub fn new(msg_iov: u64, msg_iovlen: u64) -> Self {
		Self { msg_iov, msg_iovlen }
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let max = min(ret, MAX_PAYLOAD_SIZE) as usize;
		let round = min(self.msg_iovlen as usize, IOV_MAX);
		let mut offset: usize = 0;
		let msg_iov = self.msg_iov as *mut iovec;
		for i in 0..round {
			if offset >= max {
				break;
			}
			let iovec = unsafe {
				let iovec_ptr = msg_iov.add(i);
				let mut v: MaybeUninit<iovec> = MaybeUninit::uninit();
				if bpf_probe_read(
					v.as_mut_ptr() as *mut _,
					size_of::<iovec>() as u32,
					iovec_ptr as *const _,
				) != 0
				{
					return Err(0);
				};
				v.assume_init()
			};

			let copy_size = min(iovec.iov_len as u32, (max - offset) as u32);
			if copy_size >= MAX_IOVEC_PAYLOAD_SIZE {
				if unsafe {
					bpf_probe_read(
						buf.add(offset) as *mut _,
						MAX_IOVEC_PAYLOAD_SIZE,
						iovec.iov_base as *const _,
					)
				} != 0
				{
					return Err(0);
				}
				offset += MAX_IOVEC_PAYLOAD_SIZE as usize;
			} else {
				if unsafe {
					bpf_probe_read(
						buf.add(offset) as *mut _,
						copy_size & IOVEC_MASK,
						iovec.iov_base as *const _,
					)
				} != 0
				{
					return Err(0);
				}
				offset += (copy_size & IOVEC_MASK) as usize;
			};
		}
		Ok(offset as u32)
	}
	#[inline]
	pub fn infer_extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let max = min(ret, MAX_INFER_PAYLOAD_SIZE) as usize;
		let round = min(self.msg_iovlen as usize, IOV_MAX);
		let mut offset: usize = 0;
		let msg_iov = self.msg_iov as *mut iovec;
		for i in 0..round {
			if offset >= max {
				break;
			}
			let iovec = unsafe {
				let iovec_ptr = msg_iov.add(i);
				let mut v: MaybeUninit<iovec> = MaybeUninit::uninit();
				if bpf_probe_read(
					v.as_mut_ptr() as *mut _,
					size_of::<iovec>() as u32,
					iovec_ptr as *const _,
				) != 0
				{
					return Err(0);
				};
				v.assume_init()
			};

			let copy_size = min(iovec.iov_len as u32, (max - offset) as u32);
			let copy_size = if copy_size >= MAX_INFER_PAYLOAD_SIZE {
				MAX_INFER_PAYLOAD_SIZE
			} else {
				copy_size & INFER_MASK
			};
			if unsafe {
				bpf_probe_read(
					buf.add(offset) as *mut _,
					copy_size & INFER_MASK,
					iovec.iov_base as *const _,
				)
			} != 0
			{
				return Err(0);
			}
			offset += copy_size as usize;
		}
		Ok(offset as u32)
	}
	#[inline]
	pub fn save_prev(&self, buf: *mut u8) -> Result<u32, u32> {
		let mut offset: u32 = 0;
		let msg_iov = self.msg_iov as *mut iovec;
		let iovec = unsafe {
			let iovec_ptr = msg_iov.add(0);
			let mut v: MaybeUninit<iovec> = MaybeUninit::uninit();
			if bpf_probe_read(
				v.as_mut_ptr() as *mut _,
				size_of::<iovec>() as u32,
				iovec_ptr as *const _,
			) != 0
			{
				return Err(0);
			};
			v.assume_init()
		};

		if iovec.iov_len as u32 >= MAX_INFER_PAYLOAD_SIZE {
			if unsafe {
				bpf_probe_read(buf as *mut _, MAX_INFER_PAYLOAD_SIZE, iovec.iov_base as *const _)
			} != 0
			{
				return Err(0);
			}
			offset += MAX_INFER_PAYLOAD_SIZE;
		} else {
			if unsafe {
				bpf_probe_read(
					buf as *mut _,
					(iovec.iov_len as u32) & INFER_MASK,
					iovec.iov_base as *const _,
				)
			} != 0
			{
				return Err(0);
			}
			offset += (iovec.iov_len as u32) & INFER_MASK;
		};
		Ok(offset)
	}
}
#[derive(Copy, Clone)]
pub(crate) struct MsgBuffer {
	mmsg: u64,
	vlen: u64,
}
impl MsgBuffer {
	#[inline]
	pub fn new(mmsg: u64, vlen: u64) -> Self {
		Self { mmsg, vlen }
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let max = MAX_PAYLOAD_SIZE as usize;
		let round = min(IOVLEN_MAX, self.vlen as usize);
		let mut remain_msg = ret;
		let mut offset: usize = 0;
		let mmsg = self.mmsg as *mut mmsghdr;
		for i in 0..round {
			if offset >= max || remain_msg == 0 {
				break;
			}
			let mmsghdr = unsafe {
				let mmsghdr_ptr = mmsg.add(i);
				let mut v: MaybeUninit<mmsghdr> = MaybeUninit::uninit();
				if bpf_probe_read(
					v.as_mut_ptr() as *mut _,
					size_of::<mmsghdr>() as u32,
					mmsghdr_ptr as *const _,
				) != 0
				{
					return Err(0);
				};
				v.assume_init()
			};
			// TODO: how to handle re-sendmmsg? in which case that msghdr.msg_len is `not` zero
			// let mut inside_offset = mmsghdr.msg_len;
			let msg_hdr = mmsghdr.msg_hdr;
			let msg_iovlen = msg_hdr.msg_iovlen as usize;
			let inside_round = min(remain_msg as usize, min(msg_iovlen, IOV_MAX));
			for j in 0..inside_round {
				if offset >= max {
					break;
				}
				let iovec = unsafe {
					let iovec_ptr = msg_hdr.msg_iov.add(j);
					let mut v: MaybeUninit<iovec> = MaybeUninit::uninit();
					if bpf_probe_read(
						v.as_mut_ptr() as *mut _,
						size_of::<iovec>() as u32,
						iovec_ptr as *const _,
					) != 0
					{
						return Err(0);
					};
					v.assume_init()
				};
				let copy_size = min(iovec.iov_len as u32, (max - offset) as u32);
				if copy_size >= MAX_IOVEC_PAYLOAD_SIZE {
					if unsafe {
						bpf_probe_read(
							buf.add(offset) as *mut _,
							MAX_IOVEC_PAYLOAD_SIZE,
							iovec.iov_base as *const _,
						)
					} != 0
					{
						return Err(0);
					}
					offset += MAX_IOVEC_PAYLOAD_SIZE as usize;
				} else {
					if unsafe {
						bpf_probe_read(
							buf.add(offset) as *mut _,
							copy_size & IOVEC_MASK,
							iovec.iov_base as *const _,
						)
					} != 0
					{
						return Err(0);
					}
					offset += (copy_size & IOVEC_MASK) as usize;
				};
			}
			remain_msg -= inside_round as u32;
		}
		Ok(offset as u32)
	}
	#[inline]
	pub fn infer_extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		let max = MAX_INFER_PAYLOAD_SIZE as usize;
		let round = min(IOVLEN_MAX, self.vlen as usize);
		let mut remain_msg = ret;
		let mut offset: usize = 0;
		let mmsg = self.mmsg as *mut mmsghdr;
		for i in 0..round {
			if offset >= max || remain_msg == 0 {
				break;
			}
			let mmsghdr = unsafe {
				let mmsghdr_ptr = mmsg.add(i);
				let mut v: MaybeUninit<mmsghdr> = MaybeUninit::uninit();
				if bpf_probe_read(
					v.as_mut_ptr() as *mut _,
					size_of::<mmsghdr>() as u32,
					mmsghdr_ptr as *const _,
				) != 0
				{
					return Err(0);
				};
				v.assume_init()
			};
			let msg_hdr = mmsghdr.msg_hdr;
			let msg_iovlen = msg_hdr.msg_iovlen as usize;
			let inside_round = min(remain_msg as usize, min(msg_iovlen, IOV_MAX));
			for j in 0..inside_round {
				if offset >= max {
					break;
				}
				let iovec = unsafe {
					let iovec_ptr = msg_hdr.msg_iov.add(j);
					let mut v: MaybeUninit<iovec> = MaybeUninit::uninit();
					if bpf_probe_read(
						v.as_mut_ptr() as *mut _,
						size_of::<iovec>() as u32,
						iovec_ptr as *const _,
					) != 0
					{
						return Err(0);
					};
					v.assume_init()
				};
				let copy_size = min(iovec.iov_len as u32, (max - offset) as u32);
				if copy_size >= MAX_INFER_PAYLOAD_SIZE {
					if unsafe {
						bpf_probe_read(
							buf.add(offset) as *mut _,
							MAX_INFER_PAYLOAD_SIZE,
							iovec.iov_base as *const _,
						)
					} != 0
					{
						return Err(0);
					}
					offset += MAX_INFER_PAYLOAD_SIZE as usize;
				} else {
					if unsafe {
						bpf_probe_read(
							buf.add(offset) as *mut _,
							copy_size & INFER_MASK,
							iovec.iov_base as *const _,
						)
					} != 0
					{
						return Err(0);
					}
					offset += (copy_size & INFER_MASK) as usize;
				};
			}
			remain_msg -= inside_round as u32;
		}
		Ok(offset as u32)
	}
	#[inline]
	pub fn save_prev(&self, buf: *mut u8) -> Result<u32, u32> {
		let mut offset: u32 = 0;
		let mmsg = self.mmsg as *mut mmsghdr;
		let mmsghdr = unsafe {
			let mmsghdr_ptr = mmsg.add(0);
			let mut v: MaybeUninit<mmsghdr> = MaybeUninit::uninit();
			if bpf_probe_read(
				v.as_mut_ptr() as *mut _,
				size_of::<mmsghdr>() as u32,
				mmsghdr_ptr as *const _,
			) != 0
			{
				return Err(0);
			};
			v.assume_init()
		};
		let msg_hdr = mmsghdr.msg_hdr;
		let iovec = unsafe {
			let iovec_ptr = msg_hdr.msg_iov.add(0);
			let mut v: MaybeUninit<iovec> = MaybeUninit::uninit();
			if bpf_probe_read(
				v.as_mut_ptr() as *mut _,
				size_of::<iovec>() as u32,
				iovec_ptr as *const _,
			) != 0
			{
				return Err(0);
			};
			v.assume_init()
		};
		if iovec.iov_len as u32 >= MAX_INFER_PAYLOAD_SIZE {
			if unsafe {
				bpf_probe_read(buf as *mut _, MAX_INFER_PAYLOAD_SIZE, iovec.iov_base as *const _)
			} != 0
			{
				return Err(0);
			}
			offset += MAX_INFER_PAYLOAD_SIZE;
		} else {
			if unsafe {
				bpf_probe_read(
					buf as *mut _,
					(iovec.iov_len as u32) & INFER_MASK,
					iovec.iov_base as *const _,
				)
			} != 0
			{
				return Err(0);
			}
			offset += (iovec.iov_len as u32) & INFER_MASK;
		};
		Ok(offset)
	}
}

pub(crate) struct InferInfo {
	/// SocketInfo key
	pub key: u64,
	/// Syscall return value
	pub count: u32,
	/// Parsed payload length
	pub len: u32,
	pub enter_seq: u32,
	pub exit_seq: u32,
	pub direction: Direction,
	padding: u8,
	/// Payload to be parsed
	pub buf: [u8; MAX_INFER_PAYLOAD_SIZE as usize * IOV_MAX],
}
