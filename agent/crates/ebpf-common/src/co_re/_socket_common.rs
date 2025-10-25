use super::{
	CoRe,
	generate::{
		self, shim_sock_common_skc_addrpair, shim_sock_common_skc_addrpair_exists,
		shim_sock_common_skc_family, shim_sock_common_skc_family_exists,
		shim_sock_common_skc_ipv6only, shim_sock_common_skc_ipv6only_exists,
		shim_sock_common_skc_portpair, shim_sock_common_skc_portpair_exists,
		shim_sock_common_skc_state, shim_sock_common_skc_state_exists,
	},
};
use crate::macros::kernel_shim;

pub type sock_common = CoRe<generate::sock_common>;

#[repr(C)]
#[allow(non_camel_case_types)]
struct skc_addrpair {
	skc_daddr: u32,
	skc_rcv_saddr: u32,
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct skc_portpair {
	skc_dport: u16,
	skc_num: u16,
}

impl sock_common {
	kernel_shim!(pub, sock_common, skc_addrpair, u64);

	#[inline(always)]
	pub fn skc_daddr(&self) -> Option<u32> {
		let addrpair: skc_addrpair = unsafe { core::mem::transmute(self.skc_addrpair()?) };
		Some(addrpair.skc_daddr)
	}

	#[inline(always)]
	pub fn skc_rcv_saddr(&self) -> Option<u32> {
		let addrpair: skc_addrpair = unsafe { core::mem::transmute(self.skc_addrpair()?) };
		Some(addrpair.skc_rcv_saddr)
	}

	kernel_shim!(pub, sock_common, skc_portpair, u32);

	#[inline(always)]
	pub fn skc_dport(&self) -> Option<u16> {
		let portpair: skc_portpair = unsafe { core::mem::transmute(self.skc_portpair()?) };
		Some(portpair.skc_dport)
	}

	#[inline(always)]
	pub fn skc_num(&self) -> Option<u16> {
		let portpair: skc_portpair = unsafe { core::mem::transmute(self.skc_portpair()?) };
		Some(portpair.skc_num)
	}

	kernel_shim!(pub, sock_common, skc_family, u16);
	kernel_shim!(pub, sock_common, skc_state, u8);
	kernel_shim!(pub, sock_common, skc_ipv6only, u8);
}
