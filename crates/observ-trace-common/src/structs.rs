use crate::protocols::L4Protocol;
use ebpf_common::constants::MAX_PAYLOAD_SIZE;
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
	#[serde(skip)]
	padding: u16,
}

impl Quintuple {
	#[inline(always)]
	pub fn new(
		src_addr: u32,
		dst_addr: u32,
		src_port: u16,
		dst_port: u16,
		l4_protocol: u16,
	) -> Quintuple {
		Self {
			src_addr,
			dst_addr,
			src_port,
			dst_port,
			l4_protocol: unsafe { core::mem::transmute(l4_protocol) },
			padding: 0,
		}
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
	pub buf: [u8; MAX_PAYLOAD_SIZE],
}
