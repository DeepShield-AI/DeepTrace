use super::{
	CoRe,
	generate::{
		self, shim_inet_sock_inet_saddr, shim_inet_sock_inet_saddr_exists,
		shim_inet_sock_inet_sport, shim_inet_sock_inet_sport_exists, shim_inet_sock_sk,
		shim_inet_sock_sk_exists,
	},
	sock,
};
use crate::macros::kernel_shim;

pub type inet_sock = CoRe<generate::inet_sock>;

impl inet_sock {
	kernel_shim!(pub, inet_sock, sk, sock);
	kernel_shim!(pub, inet_sock, inet_saddr, u32);
	kernel_shim!(pub, inet_sock, inet_sport, u16);
}
