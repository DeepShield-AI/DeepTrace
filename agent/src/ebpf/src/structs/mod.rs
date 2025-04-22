mod buffer;

use buffer::Buffer;
pub(crate) use buffer::InferInfo;
use mercury_common::{consts::MAX_INFER_PAYLOAD_SIZE, protocols::L7Protocol, structs::Direction};

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

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SocketInfo {
	pub exit_seq: u32,
	pub direction: Direction,
	pub pre_direction: Direction,
	pub l7protocol: L7Protocol,
	padding: u8,
	pub prev_len: u32,
	pub prev_buf: [u8; MAX_INFER_PAYLOAD_SIZE as usize],
}

impl SocketInfo {
	pub fn new() -> Self {
		Self {
			exit_seq: 0,
			pre_direction: Direction::Unknown,
			direction: Direction::Unknown,
			l7protocol: L7Protocol::Unknown,
			padding: 0,
			prev_len: 0,
			prev_buf: [0; MAX_INFER_PAYLOAD_SIZE as usize],
		}
	}
}
