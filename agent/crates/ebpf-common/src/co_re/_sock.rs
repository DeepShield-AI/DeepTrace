use super::{
	CoRe,
	generate::{
		self, shim_sock___sk_common, shim_sock___sk_common_exists, shim_sock_sk_type,
		shim_sock_sk_type_exists,
	},
	sock_common,
};
use crate::macros::kernel_shim;

pub type sock = CoRe<generate::sock>;

impl sock {
	kernel_shim!(pub, sock, __sk_common, sock_common);
	kernel_shim!(pub, sock, sk_type, u16);
}
