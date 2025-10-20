use crate::{direction::Direction, protocols::L7Protocol, socket::Quintuple, syscall::Syscall};
use aya_ebpf::TASK_COMM_LEN;
pub use ebpf_common::{buffer::Buffer, constants::MAX_PAYLOAD_SIZE};

#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MessageType {
	Unknown = 0,
	Request = 1,
	Response = 2,
}

#[cfg(feature = "user")]
impl std::fmt::Display for MessageType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MessageType::Unknown => f.write_str("Unknown"),
			MessageType::Request => f.write_str("Request"),
			MessageType::Response => f.write_str("Response"),
		}
	}
}

/// Syscall message sent to user space
#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[repr(C)]
pub struct Message {
	pub tgid: u32,
	pub pid: u32,
	// TODO: are enter and exit seq necessary?
	pub enter_seq: u32,
	pub exit_seq: u32,
	#[cfg_attr(feature = "user", serde(skip))]
	pub seq: u32,
	// TODO: this timestamp is enter syscall time, do we need to add more timestamps?
	pub timestamp_ns: u64,
	// this is needed for span constructor, but this field should not deliver to sender, so we skip serialize it
	#[cfg_attr(feature = "user", serde(skip))]
	pub uuid: u32,
	#[cfg_attr(feature = "user", serde(flatten))]
	pub quintuple: Quintuple,
	pub syscall: Syscall,
	pub direction: Direction,
	#[cfg_attr(feature = "user", serde(rename(serialize = "type")))]
	pub type_: MessageType,
	pub protocol: L7Protocol,
	#[cfg_attr(feature = "user", serde(serialize_with = "serialize_comm"))]
	pub comm: Buffer<TASK_COMM_LEN>,
	#[cfg_attr(feature = "user", serde(serialize_with = "serialize_buffer"))]
	pub payload: Buffer<MAX_PAYLOAD_SIZE>,
}

impl Message {
	#[inline]
	pub fn encode(&self) -> &[u8] {
		unsafe {
			core::slice::from_raw_parts(
				(self as *const Self) as *const u8,
				core::mem::size_of::<Message>(),
			)
		}
	}

	#[cfg(feature = "user")]
	#[inline]
	pub fn decode(bytes: &[u8]) -> Self {
		const LENGTH: usize = core::mem::size_of::<Message>();
		assert!(LENGTH <= bytes.len(), "Not enough bytes to decode Message");

		let mut buf = [0u8; LENGTH];
		buf.copy_from_slice(&bytes[0..LENGTH]);
		unsafe { core::mem::transmute::<[u8; LENGTH], Message>(buf) }
	}
}

#[cfg(feature = "user")]
impl Message {
	#[inline]
	pub fn is_request(&self) -> bool {
		self.type_ == MessageType::Request
	}
	#[inline]
	pub fn is_response(&self) -> bool {
		self.type_ == MessageType::Response
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for Message {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"tgid: {}, pid: {}, Quintuple: {}, Time: {}, Command: {}, Syscall: {}, Direction: {}, Protocol: {}, Type: {}, Enter: {}, Exit: {}, Data: {}",
			self.tgid,
			self.pid,
			self.quintuple,
			self.timestamp_ns,
			String::from_utf8_lossy(self.comm.as_slice()),
			self.syscall,
			self.direction,
			self.protocol,
			self.type_,
			self.enter_seq,
			self.exit_seq,
			String::from_utf8_lossy(self.payload.as_slice()),
		)
	}
}

#[cfg(feature = "user")]
fn serialize_comm<S>(i: &Buffer<TASK_COMM_LEN>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	use core::ffi::CStr;

	let s = CStr::from_bytes_until_nul(i.as_slice()).unwrap().to_str().unwrap();
	serializer.serialize_str(s)
}

#[cfg(feature = "user")]
pub fn serialize_buffer<S>(i: &Buffer<MAX_PAYLOAD_SIZE>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	serializer.serialize_str(std::str::from_utf8(i.as_slice()).unwrap())
}
