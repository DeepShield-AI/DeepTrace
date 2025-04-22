#![allow(non_camel_case_types)]
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(FromPrimitive, IntoPrimitive, Copy, Clone, PartialEq)]
#[repr(u16)]
/// Types of sockets.
pub enum SocketType {
	/// Sequenced, reliable, connection-based byte streams.
	SOCK_STREAM = 1,
	/// Connectionless, unreliable datagrams of fixed maximum length.
	SOCK_DGRAM = 2,
	/// Raw protocol interface.
	SOCK_RAW = 3,
	/// Reliably-delivered messages.
	SOCK_RDM = 4,
	/// Sequenced, reliable, connection-based, datagrams of fixed maximum length.
	SOCK_SEQPACKET = 5,
	/// Datagram Congestion Control Protocol.
	SOCK_DCCP = 6,
	/// Linux specific way of getting packets at the dev level. For writing rarp and other similar things on the user level.
	SOCK_PACKET = 10,
	/// Flags to be ORed into the type parameter of socket and socketpair and used for the flags parameter of paccept.  */
	/// Atomically set close-on-exec flag for the new descriptor(s).
	// FIXME: This is not a valid value for the enum, but it is used in the code.
	// SOCK_CLOEXEC = 0o2000000,
	/// Atomically mark descriptor(s) as non-blocking.
	SOCK_NONBLOCK = 0o00004000,
	#[num_enum(catch_all)]
	Unknown(u16),
}
