use crate::{
	constants::MAX_INFER_SIZE,
	direction::Direction,
	protocols::{L4Protocol, L7Protocol},
};
use ebpf_common::buffer::Buffer;

#[cfg_attr(feature = "user", derive(serde::Serialize, Hash, Eq, PartialEq))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quintuple {
	pub src_addr: u32,
	pub dst_addr: u32,
	pub src_port: u16,
	pub dst_port: u16,
	/// L4 protocol families. Repr(u16)
	pub l4_protocol: L4Protocol,
	#[cfg_attr(feature = "user", serde(skip))]
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
			l4_protocol: unsafe { core::mem::transmute::<u16, L4Protocol>(l4_protocol) },
			padding: 0,
		}
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for Quintuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"l4_protocol: {}, saddr: {}, daddr: {}, sport: {}, dport: {}",
			self.l4_protocol,
			std::net::Ipv4Addr::from(self.src_addr.to_be_bytes()),
			std::net::Ipv4Addr::from(self.dst_addr.to_be_bytes()),
			self.src_port,
			self.dst_port,
		))
	}
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SocketInfo {
	pub uuid: u32,
	pub exit_seq: u32,
	pub seq: u32,
	pub direction: Direction,
	pub pre_direction: Direction,
	pub l7protocol: L7Protocol,
	padding: u8,
	pub prev_buf: Buffer<MAX_INFER_SIZE>,
}
