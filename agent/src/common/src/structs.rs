use super::{
	consts::{MAX_PAYLOAD_SIZE, TASK_COMM_LEN},
	protocols::{L4Protocol, L7Protocol},
};
use crate::message::MessageType;

#[repr(u8)]
#[derive(Copy, Clone)]
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

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct Data {
	pub tgid: u32,
	pub pid: u32,
	pub enter_seq: u32,
	pub exit_seq: u32,
	pub timestamp_ns: u64,
	pub len: u32,

	pub quintuple: Quintuple,
	pub syscall: Syscall,
	pub direction: Direction,

	pub comm: [u8; TASK_COMM_LEN],

	pub buf: [u8; MAX_PAYLOAD_SIZE as usize],
	// for protocol infer
	pub protocol: L7Protocol,
	// 请求或者响应
	pub type_: MessageType,
	// 需要解析协议
	pub require_recheck: bool,
}

#[cfg(feature = "user")]
impl std::fmt::Display for Data {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
            f,
            "tgid: {:?}, Time: {:?}, Command: {:?}, Syscall: {}, Direction: {:?}, Length: {}, Data: {:?}",
            self.tgid,
            self.timestamp_ns,
            // Convert 'cmd' and 'buf' fields to strings for display.
            // 'String::from_utf8_lossy' will replace invalid UTF-8 sequences with U+FFFD REPLACEMENT CHARACTER.
            String::from_utf8_lossy(&self.comm),
            self.syscall,
            self.direction,
            self.len,
            String::from_utf8_lossy(&self.buf[..self.len as usize]),
        )
	}
}

#[cfg(feature = "user")]
impl Data {
	pub fn buffer(&self) -> Vec<u8> {
		self.buf[..self.len as usize].to_vec()
		// self.pre_payload[..self.pre_len as usize]
		// 	.iter()
		// 	.chain(self.buf[..self.len as usize].iter())
		// 	.copied()
		// 	.collect()
	}
}
