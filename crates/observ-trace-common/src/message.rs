use crate::{
	protocols::L7Protocol,
	structs::{Direction, Quintuple, Syscall},
};
pub use ebpf_common::{buffer::Buffer, constants::MAX_PAYLOAD_SIZE};
use serde::Serialize;

/// Syscall message sent to user space
#[cfg_attr(feature = "user", derive(Serialize))]
#[repr(C)]
pub struct Message {
	pub tgid: u32,
	pub pid: u32,
	// TODO: are enter and exit seq necessary?
	pub enter_seq: u32,
	pub exit_seq: u32,
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
	#[cfg_attr(feature = "user", serde(skip))]
	padding: u16,
	#[cfg_attr(feature = "user", serde(serialize_with = "serialize_comm"))]
	pub comm: [u8; 16],
	#[cfg_attr(feature = "user", serde(serialize_with = "serialize_buffer"))]
	pub payload: Buffer<MAX_PAYLOAD_SIZE>,
	// for protocol infer
	pub protocol: L7Protocol,
	#[cfg_attr(feature = "user", serde(rename(serialize = "type")))]
	pub type_: MessageType,
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

#[derive(Default, Clone, Copy, Serialize, Debug, PartialEq)]
pub enum MessageType {
	#[default]
	Unknown = -1,
	Request = 0,
	Response = 1,
}

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
			"tgid: {}, pid: {}, Quintuple: {}, Time: {}, Command: {}, Syscall: {}, Direction: {}, Protocol: {}, Type: {:?}, Enter: {}, Exit: {}, Data: {}",
			self.tgid,
			self.pid,
			self.quintuple,
			self.timestamp_ns,
			String::from_utf8_lossy(&self.comm),
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
impl Message {
	pub fn buffer(&self) -> &[u8] {
		self.payload.as_slice()
	}
}

#[cfg(feature = "user")]
fn serialize_comm<S>(i: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	use core::ffi::CStr;

	let s = CStr::from_bytes_until_nul(i).unwrap().to_str().unwrap();
	serializer.serialize_str(s)
}

#[cfg(feature = "user")]
pub fn serialize_buffer<S>(i: &Buffer<MAX_PAYLOAD_SIZE>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	serializer.serialize_str(std::str::from_utf8(i.as_slice()).unwrap())
}
