mod buffer;
mod socket;

use buffer::Buffer;
pub(crate) use buffer::InferInfo;
pub(crate) use socket::SocketInfo;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Args {
	pub fd: u64,
	pub timestamp: u64,
	// quintuple: Quintuple,
	pub buffer: Buffer,
	pub enter_seq: u32,
	pub padding: u32,
}
impl Args {
	#[inline]
	pub fn new(fd: u64, timestamp: u64, buffer: Buffer, enter_seq: u32) -> Self {
		Self { fd, buffer, timestamp, enter_seq, padding: 0 }
	}
	#[inline]
	pub fn normal(fd: u64, buffer: u64, count: u64, timestamp: u64, enter_seq: u32) -> Self {
		let buffer = Buffer::normal(buffer, count);
		Self::new(fd, timestamp, buffer, enter_seq)
	}
	#[inline]
	pub fn vectored(
		fd: u64,
		msg_iov: u64,
		msg_iovlen: u64,
		timestamp: u64,
		enter_seq: u32,
	) -> Self {
		let buffer = Buffer::vectored(msg_iov, msg_iovlen);
		Self::new(fd, timestamp, buffer, enter_seq)
	}
	#[inline]
	pub fn msg(fd: u64, msg_buffer: u64, vlen: u64, timestamp: u64, enter_seq: u32) -> Self {
		let buffer = Buffer::msg(msg_buffer, vlen);
		Self::new(fd, timestamp, buffer, enter_seq)
	}
	#[inline]
	pub fn extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		match &self.buffer {
			Buffer::Normal(normal) => normal.extract(buf, ret),
			Buffer::Vectored(vectored) => vectored.extract(buf, ret),
			Buffer::Msg(msg) => msg.extract(buf, ret),
		}
	}
	#[inline]
	pub fn infer_extract(&self, buf: *mut u8, ret: u32) -> Result<u32, u32> {
		match &self.buffer {
			Buffer::Normal(normal) => normal.infer_extract(buf, ret),
			Buffer::Vectored(vectored) => vectored.infer_extract(buf, ret),
			Buffer::Msg(msg) => msg.infer_extract(buf, ret),
		}
	}
	#[inline]
	pub fn save_prev(&self, buf: *mut u8) -> Result<u32, u32> {
		match &self.buffer {
			Buffer::Normal(normal) => normal.save_prev(buf),
			Buffer::Vectored(vectored) => vectored.save_prev(buf),
			Buffer::Msg(msg) => msg.save_prev(buf),
		}
	}
}
