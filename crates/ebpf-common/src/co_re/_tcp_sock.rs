use super::{
	CoRe,
	generate::{
		self, shim_tcp_sock_copied_seq, shim_tcp_sock_copied_seq_exists, shim_tcp_sock_inet_conn,
		shim_tcp_sock_inet_conn_exists, shim_tcp_sock_write_seq, shim_tcp_sock_write_seq_exists,
	},
	inet_connection_sock,
};
use crate::macros::kernel_shim;

pub type tcp_sock = CoRe<generate::tcp_sock>;

impl tcp_sock {
	kernel_shim!(pub, tcp_sock, inet_conn, inet_connection_sock);
	kernel_shim!(pub, tcp_sock, copied_seq, u32);
	kernel_shim!(pub, tcp_sock, write_seq, u32);
}
