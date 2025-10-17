use super::{
	CoRe,
	generate::{
		self, shim_inet_connection_sock_icsk_inet, shim_inet_connection_sock_icsk_inet_exists,
	},
	inet_sock,
};
use crate::macros::kernel_shim;

pub type inet_connection_sock = CoRe<generate::inet_connection_sock>;

impl inet_connection_sock {
	kernel_shim!(pub, inet_connection_sock, icsk_inet, inet_sock);
}
