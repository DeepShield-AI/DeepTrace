use super::{
	CoRe,
	generate::{
		self, shim_socket_sk, shim_socket_sk_exists, shim_socket_type, shim_socket_type_exists,
	},
	sock,
};
use crate::macros::kernel_shim;
use num_enum::FromPrimitive;

pub type socket = CoRe<generate::socket>;

impl socket {
	kernel_shim!(pub, type_, socket, type, i16);
	kernel_shim!(pub, socket, sk, sock);
}

impl socket {
	pub fn is_valid_type(&self) -> bool {
		self.type_().is_some() &&
			(sock_type::from(self.type_().unwrap()) == sock_type::SOCK_STREAM ||
				sock_type::from(self.type_().unwrap()) == sock_type::SOCK_DGRAM)
	}
}

#[derive(FromPrimitive, PartialEq)]
#[repr(i16)]
pub enum sock_type {
	SOCK_STREAM = 1,
	SOCK_DGRAM = 2,
	SOCK_RAW = 3,
	SOCK_RDM = 4,
	SOCK_SEQPACKET = 5,
	SOCK_DCCP = 6,
	SOCK_PACKET = 10,
	#[num_enum(catch_all)]
	Unknown(i16),
}
