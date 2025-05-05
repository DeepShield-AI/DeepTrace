use crate::{
	constants::{MAX_PAYLOAD_SIZE, TASK_COMM_LEN},
	message::MessageType,
	protocols::{L4Protocol, L7Protocol},
};
use serde::Serialize;

#[derive(Copy, Clone, Serialize, Debug)]
#[repr(u8)]
pub enum Syscall {
	Read,
	RecvMsg,
	RecvMMsg,
	ReadV,
	RecvFrom,
	Write,
	SendMsg,
	SendMMsg,
	SendTo,
	WriteV,
	Unknown,
}

impl From<&Syscall> for &'static str {
	fn from(syscall: &Syscall) -> Self {
		match syscall {
			Syscall::Read => "read",
			Syscall::RecvMsg => "recvmsg",
			Syscall::RecvMMsg => "recvmmsg",
			Syscall::ReadV => "readv",
			Syscall::RecvFrom => "recvfrom",
			Syscall::Write => "write",
			Syscall::SendMsg => "sendmsg",
			Syscall::SendMMsg => "sendmmsg",
			Syscall::SendTo => "sendto",
			Syscall::WriteV => "writev",
			Syscall::Unknown => "unknown",
		}
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for Syscall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.into())
	}
}

#[derive(Copy, Clone, PartialEq, Serialize, Debug)]
#[repr(u8)]
pub enum Direction {
	Ingress,
	Egress,
	Unknown,
}

impl From<&Direction> for &'static str {
	fn from(direction: &Direction) -> Self {
		match direction {
			Direction::Ingress => "ingress",
			Direction::Egress => "egress",
			Direction::Unknown => "unknown",
		}
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.into())
	}
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Debug)]
#[repr(C)]
pub struct Quintuple {
	pub src_addr: u32,
	pub dst_addr: u32,
	pub src_port: u16,
	pub dst_port: u16,
	/// L4 protocol families. Repr(u16)
	pub l4_protocol: L4Protocol,
}

impl Quintuple {
	pub fn new(
		src_addr: u32,
		dst_addr: u32,
		src_port: u16,
		dst_port: u16,
		l4_protocol: u16,
	) -> Quintuple {
		Self { src_addr, dst_addr, src_port, dst_port, l4_protocol: L4Protocol::from(l4_protocol) }
	}

	pub fn protocol(&self) -> &'static str {
		(&self.l4_protocol).into()
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for Quintuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"l4_protocol: {}, saddr: {}, daddr: {}, sport: {}, dport: {}",
			self.protocol(),
			std::net::Ipv4Addr::from(self.src_addr.to_be_bytes()),
			std::net::Ipv4Addr::from(self.dst_addr.to_be_bytes()),
			self.src_port,
			self.dst_port,
		))
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Payload {
	pub len: u32,
	pub buf: [u8; MAX_PAYLOAD_SIZE as usize],
}

#[cfg_attr(feature = "user", derive(Serialize))]
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Data {
	pub tgid: u32,
	pub pid: u32,
	pub enter_seq: u32,
	pub exit_seq: u32,
	pub timestamp_ns: u64,
	pub uuid: u32,
	// #[serde(flatten)]
	pub quintuple: Quintuple,
	pub syscall: Syscall,
	pub direction: Direction,
	#[cfg_attr(feature = "user", serde(serialize_with = "serialize_comm"))]
	pub comm: [u8; TASK_COMM_LEN],
	#[cfg_attr(feature = "user", serde(serialize_with = "serialize_payload"))]
	pub payload: Payload,
	// for protocol infer
	pub protocol: L7Protocol,
	// 请求或者响应
	#[cfg_attr(feature = "user", serde(rename(serialize = "type")))]
	pub type_: MessageType,
}

#[cfg(feature = "user")]
impl std::fmt::Display for Data {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"tgid: {}, pid: {}, Quintuple: {}, Time: {}, Command: {}, Syscall: {}, Direction: {}, Length: {}, Protocol: {}, Type: {}, UUID: {}, Enter: {}, Exit: {}, Data: {}",
			self.tgid,
			self.pid,
			self.quintuple,
			self.timestamp_ns,
			String::from_utf8_lossy(&self.comm),
			self.syscall,
			self.direction,
			self.payload.len,
			self.protocol,
			self.type_,
			self.uuid,
			self.enter_seq,
			self.exit_seq,
			String::from_utf8_lossy(&self.payload.buf[..self.payload.len as usize]),
		)
	}
}
#[cfg(feature = "user")]
impl Data {
	pub fn buffer(&self) -> Vec<u8> {
		self.payload.buf[..self.payload.len as usize].to_vec()
	}
}
#[cfg(feature = "user")]
fn serialize_comm<S>(i: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	let s = String::from_utf8_lossy(i);
	serializer.serialize_str(&s)
}

#[cfg(feature = "user")]
fn serialize_payload<S>(payload: &Payload, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	let s = String::from_utf8_lossy(&payload.buf[..payload.len as usize]);
	serializer.serialize_str(&s)
}
