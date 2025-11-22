use crate::{
	co_re::{iovec, mmsghdr},
	constants::IOV_MAX,
	error::{Result, code::*},
};
use aya_ebpf::{
	check_bounds_signed,
	helpers::{bpf_probe_read_kernel_str_bytes, r#gen},
};
use core::cmp::min;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Buffer<const N: usize> {
	buf: [u8; N],
	len: usize,
}

impl<const N: usize> Default for Buffer<N> {
	fn default() -> Self {
		Self { buf: [0; N], len: 0 }
	}
}

impl<const N: usize> core::ops::Index<usize> for Buffer<N> {
	type Output = u8;
	fn index(&self, index: usize) -> &Self::Output {
		&self.buf[index]
	}
}

impl<const N: usize> Buffer<N> {
	pub fn new() -> Self {
		Default::default()
	}

	#[inline(always)]
	pub fn as_slice(&self) -> &[u8] {
		&self.buf[..min(self.len(), N)]
	}

	pub fn from_slice(slice: &[u8]) -> Self {
		let mut buffer = Self::new();
		let len = min(slice.len(), N);
		buffer.buf[..len].copy_from_slice(&slice[..len]);
		buffer.len = len;
		buffer
	}

	#[inline(always)]
	pub const fn len(&self) -> usize {
		self.len
	}

	#[inline(always)]
	pub const fn is_empty(&self) -> bool {
		self.len == 0
	}

	#[inline(always)]
	pub const fn is_full(&self) -> bool {
		self.space_left() == 0
	}

	#[inline(always)]
	pub const fn space_left(&self) -> usize {
		N - self.len
	}

	#[inline(always)]
	pub fn reset(&mut self) {
		for i in 0..N {
			if i == self.len {
				break;
			}
			self.buf[i] = 0;
		}
		self.len = 0;
	}

	#[inline(always)]
	pub const fn cap(&self) -> usize {
		N
	}

	#[inline(always)]
	pub fn append(&mut self, other: &[u8]) -> Result<()> {
		// offset is at current len to append
		let offset = self.len as i64;

		let mut size = other.len() as i64;

		let left = N as i64 - offset;
		if size > left {
			// TODO: logic check here:
			size = left
		}

		// check map access is not OOB
		if !check_bounds_signed(offset, 0, N as i64) {
			return Ok(());
		}

		// check not write OOB
		if !check_bounds_signed(size, 0, N as i64) {
			return Ok(());
		}

		if let Some(dst) = self
			.buf
			// we need to clamp as we cast offset and bounds might be lost by verifier
			.get_mut((offset as usize).clamp(0, N)..N)
		{
			if let Some(src) = other.get(..(size as usize).clamp(0, N)) {
				dst.copy_from_slice(src);
				self.len += size as usize;
			}
		}
		Ok(())
	}
}

impl<const N: usize> Buffer<N> {
	#[inline(always)]
	pub fn fill_from_iovec<const IOV_MAX: usize>(
		&mut self,
		iovec: iovec,
		vlen: u32,
		count: Option<usize>,
	) -> Result<()> {
		// put a threshold to msg_iovlen (that can be fixed from call site)
		for i in 0..IOV_MAX {
			if self.is_full() || i >= vlen as usize {
				break;
			}
			self.append_iov(iovec.get(i), count)?;
		}
		Ok(())
	}

	#[inline(always)]
	pub fn fill_from_mmsghdr<const IOVLEN_MAX: usize>(
		&mut self,
		mmsg: mmsghdr,
		vlen: u32,
		count: Option<usize>,
	) -> Result<()> {
		for i in 0..IOVLEN_MAX {
			if self.is_full() || i >= vlen as usize {
				break;
			}
			let msg = mmsg.get(i).msg_hdr().ok_or(MISSING_MMSGHDR_MSG_HDR)?;
			self.fill_from_iovec::<IOV_MAX>(
				msg.msg_iov().ok_or(MISSING_USER_MSGHDR_MSG_IOV)?,
				msg.msg_iovlen().ok_or(MISSING_USER_MSGHDR_MSG_IOVLEN)? as u32,
				count,
			)?;
		}

		Ok(())
	}

	#[inline(always)]
	fn append_iov(&mut self, iov: iovec, count: Option<usize>) -> Result<()> {
		let iov_len = iov.iov_len().ok_or(MISSING_IOVEC_IOVLEN)?;
		let iov_base = iov.iov_base().ok_or(MISSING_IOVEC_IOV_BASE)?;

		// offset is at current len to append
		let offset = self.len as i64;

		let mut size = iov_len as i64;

		if let Some(count) = count {
			size = min(count as i64, size);
		}

		let left = N as i64 - offset;
		if size > left {
			// TODO: logic check here:
			// when iov_len is larger than left, it should return ok and return from caller
			// return Err(BUFFER_FULL);
			size = left
		}

		// check map access is not OOB
		if !check_bounds_signed(offset, 0, N as i64) {
			return Ok(());
		}

		// check not write OOB
		if !check_bounds_signed(size, 0, N as i64) {
			return Ok(());
		}

		if let Some(dst) = self
			.buf
			// we need to clamp as we cast offset and bounds might be lost by verifier
			.get_mut((offset as usize).clamp(0, N)..N)
			.map(|d| d.as_mut_ptr())
		{
			if unsafe {
				r#gen::bpf_probe_read_user(
					dst as *mut _,
					(size as u32).clamp(0, N as u32),
					iov_base as *const _,
				)
			} < 0
			{
				return Err(READ_IOVEC_FAILED);
			}

			self.len += size as usize;
			Ok(())
		} else {
			// this path should never be taken, as we
			// bound checked everything upstream
			Err(SHOULD_NOT_HAPPEN)
		}
	}

	#[inline(always)]
	pub fn read_kernel_str<P>(&mut self, ptr: *const P) -> Result<()> {
		unsafe {
			bpf_probe_read_kernel_str_bytes(ptr as *const _, &mut self.buf)
				.map_err(|_| READ_KERNEL_FAILED)
		}?;
		Ok(())
	}

	#[inline(always)]
	pub fn read_user_at<P>(&mut self, ptr: *const P, size: u32) -> Result<()> {
		let size = (size as i64).clamp(0, N as i64);

		if check_bounds_signed(size, 0, N as i64) {
			let ret = unsafe {
				r#gen::bpf_probe_read_user(
					self.buf.as_mut_ptr() as *mut _,
					size as u32,
					ptr as *const _,
				)
			};
			if ret != 0 {
				return Err(READ_USER_FAILED);
			}
		}

		self.len = size as usize;
		Ok(())
	}

	#[inline(always)]
	pub fn read_kernel_at<P>(&mut self, ptr: *const P, size: u32) -> Result<()> {
		let size = (size as i64).clamp(0, N as i64);

		if check_bounds_signed(size, 0, N as i64) {
			let ret = unsafe {
				r#gen::bpf_probe_read_kernel(
					self.buf.as_mut_ptr() as *mut _,
					size as u32,
					ptr as *const _,
				)
			};
			if ret != 0 {
				return Err(READ_KERNEL_FAILED);
			}
		}

		self.len = size as usize;
		Ok(())
	}
}
