use core::fmt;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::Serialize;

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum SaFamily {
	/// Unspecified.
	AF_UNSPEC = 0,
	/// Unix domain sockets
	AF_UNIX = 1,
	// POSIX name for AF_UNIX
	// AF_LOCAL = 1,
	/// Internet IP Protocol
	AF_INET = 2,
	/// Amateur Radio AX.25
	AF_AX25 = 3,
	/// Novell IPX
	AF_IPX = 4,
	/// AppleTalk DDP
	AF_APPLETALK = 5,
	/// Amateur Radio NET/ROM
	AF_NETROM = 6,
	/// Multiprotocol bridge
	AF_BRIDGE = 7,
	/// ATM PVCs
	AF_ATMPVC = 8,
	/// Reserved for X.25 project
	AF_X25 = 9,
	/// IP version 6
	AF_INET6 = 10,
	/// Amateur Radio X.25 PLP
	AF_ROSE = 11,
	/// Reserved for DECnet project
	AF_DECnet = 12,
	/// Reserved for 802.2LLC project
	AF_NETBEUI = 13,
	/// Security callback pseudo AF
	AF_SECURITY = 14,
	/// PF_KEY key management API
	AF_KEY = 15,
	AF_NETLINK = 16,
	// Alias to emulate 4.4BSD
	// AF_ROUTE=AF_NETLINK ,
	/// Packet family
	AF_PACKET = 17,
	/// Ash
	AF_ASH = 18,
	/// Acorn Econet
	AF_ECONET = 19,
	/// ATM SVCs
	AF_ATMSVC = 20,
	/// RDS sockets
	AF_RDS = 21,
	/// Linux SNA Project (nutters!)
	AF_SNA = 22,
	/// IRDA sockets
	AF_IRDA = 23,
	/// PPPoX sockets
	AF_PPPOX = 24,
	/// Wanpipe API Sockets
	AF_WANPIPE = 25,
	/// Linux LLC
	AF_LLC = 26,
	/// Native InfiniBand address
	AF_IB = 27,
	/// MPLS
	AF_MPLS = 28,
	/// Controller Area Network      
	AF_CAN = 29,
	/// TIPC sockets
	AF_TIPC = 30,
	/// Bluetooth sockets
	AF_BLUETOOTH = 31,
	/// IUCV sockets
	AF_IUCV = 32,
	/// RxRPC sockets
	AF_RXRPC = 33,
	/// mISDN sockets
	AF_ISDN = 34,
	/// Phonet sockets
	AF_PHONET = 35,
	/// IEEE802154 sockets
	AF_IEEE802154 = 36,
	/// CAIF sockets
	AF_CAIF = 37,
	/// Algorithm sockets
	AF_ALG = 38,
	/// NFC sockets
	AF_NFC = 39,
	/// vSockets
	AF_VSOCK = 40,
	/// Kernel Connection Multiplexor
	AF_KCM = 41,
	/// Qualcomm IPC Router          
	AF_QIPCRTR = 42,
	/// smc sockets: reserve number for PF_SMC protocol family that reuses
	/// AF_INET address family
	AF_SMC = 43,
	/// XDP sockets
	AF_XDP = 44,
	/// Management component transport protocol
	AF_MCTP = 45,
	/// For now.
	AF_MAX = 46,
	#[num_enum(catch_all)]
	AF_Reversed(u16),
}

impl From<&SaFamily> for &'static str {
	fn from(protocol: &SaFamily) -> Self {
		match protocol {
			SaFamily::AF_UNSPEC => "Unspecified",
			SaFamily::AF_UNIX => "Unix domain sockets",
			SaFamily::AF_INET => "IPv4",
			SaFamily::AF_AX25 => "Amateur Radio AX.25",
			SaFamily::AF_IPX => "Novell Internet Protocol",
			SaFamily::AF_APPLETALK => "Appletalk DDP",
			SaFamily::AF_NETROM => "Amateur radio NetROM",
			SaFamily::AF_BRIDGE => "Multiprotocol bridge",
			SaFamily::AF_ATMPVC => "ATM PVCs",
			SaFamily::AF_X25 => "Reserved for X.25 project",
			SaFamily::AF_INET6 => "IPv6",
			SaFamily::AF_ROSE => "Amateur Radio X.25 PLP",
			SaFamily::AF_DECnet => "Reserved for DECnet project",
			SaFamily::AF_NETBEUI => "Reserved for 802.2LLC project",
			SaFamily::AF_SECURITY => "Security callback pseudo AF",
			SaFamily::AF_KEY => "AF_KEY key management API",
			SaFamily::AF_NETLINK => "Netlink",
			SaFamily::AF_PACKET => "Packet family",
			SaFamily::AF_ASH => "Ash",
			SaFamily::AF_ECONET => "Acorn Econet",
			SaFamily::AF_ATMSVC => "ATM SVCs",
			SaFamily::AF_RDS => "RDS sockets",
			SaFamily::AF_SNA => "Linux SNA Project",
			SaFamily::AF_IRDA => "IRDA sockets",
			SaFamily::AF_PPPOX => "PPPoX sockets",
			SaFamily::AF_WANPIPE => "Wanpipe API sockets",
			SaFamily::AF_LLC => "Linux LLC",
			SaFamily::AF_IB => "Native InfiniBand address",
			SaFamily::AF_MPLS => "MPLS",
			SaFamily::AF_CAN => "Controller Area Network",
			SaFamily::AF_TIPC => "TIPC sockets",
			SaFamily::AF_BLUETOOTH => "Bluetooth sockets",
			SaFamily::AF_IUCV => "IUCV sockets",
			SaFamily::AF_RXRPC => "RxRPC sockets",
			SaFamily::AF_ISDN => "mISDN sockets",
			SaFamily::AF_PHONET => "Phonet sockets",
			SaFamily::AF_IEEE802154 => "IEEE 802.15.4 sockets",
			SaFamily::AF_CAIF => "CAIF sockets",
			SaFamily::AF_ALG => "Algorithm sockets",
			SaFamily::AF_NFC => "NFC sockets",
			SaFamily::AF_VSOCK => "vSockets",
			SaFamily::AF_KCM => "Kernel Connection Multiplexor",
			SaFamily::AF_QIPCRTR => "Qualcomm IPC Router",
			SaFamily::AF_SMC => "SMC sockets",
			SaFamily::AF_XDP => "XDP sockets",
			SaFamily::AF_MCTP => "Management component transport protocol",
			SaFamily::AF_MAX => "AF_MAX For now.",
			SaFamily::AF_Reversed(_) => "Reversed",
		}
	}
}

impl fmt::Display for SaFamily {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.into())
	}
}

/// Standard well-defined IP protocols.
#[derive(Copy, Clone, PartialEq, Hash, Eq, Serialize, Debug)]
#[repr(u16)]
pub enum L4Protocol {
	/// Dummy protocol for TCP.
	IPPROTO_IP = 0,
	/// Internet Control Message Protocol.
	IPPROTO_ICMP = 1,
	/// Internet Group Management Protocol.
	IPPROTO_IGMP = 2,
	/// IPIP tunnels (older KA9Q tunnels use 94).
	IPPROTO_IPIP = 4,
	/// Transmission Control Protocol.
	IPPROTO_TCP = 6,
	/// Exterior Gateway Protocol.
	IPPROTO_EGP = 8,
	/// PUP protocol.
	IPPROTO_PUP = 12,
	/// User Datagram Protocol.
	IPPROTO_UDP = 17,
	/// XNS IDP protocol.
	IPPROTO_IDP = 22,
	/// SO Transport Protocol Class 4.
	IPPROTO_TP = 29,
	/// Datagram Congestion Control Protocol.
	IPPROTO_DCCP = 33,
	/// IPv6 header.
	IPPROTO_IPV6 = 41,
	/// Reservation Protocol.
	IPPROTO_RSVP = 46,
	/// General Routing Encapsulation.
	IPPROTO_GRE = 47,
	/// encapsulating security payload.
	IPPROTO_ESP = 50,
	/// authentication header.
	IPPROTO_AH = 51,
	/// Multicast Transport Protocol.
	IPPROTO_MTP = 92,
	/// IP option pseudo header for BEET.
	IPPROTO_BEETPH = 94,
	/// Encapsulation Header.
	IPPROTO_ENCAP = 98,
	/// Protocol Independent Multicast.
	IPPROTO_PIM = 103,
	/// Compression Header Protocol.
	IPPROTO_COMP = 108,
	/// Layer 2 Tunnelling Protocol.
	IPPROTO_L2TP = 115,
	/// Stream Control Transmission Protocol.
	IPPROTO_SCTP = 132,
	/// UDP-Lite protocol.
	IPPROTO_UDPLITE = 136,
	/// MPLS in IP.
	IPPROTO_MPLS = 137,
	/// Ethernet-within-IPv6 Encapsulation.
	IPPROTO_ETHERNET = 143,
	/// Raw IP packets.
	IPPROTO_RAW = 255,
	/// Multipath TCP connection.
	IPPROTO_MPTCP = 262,
}

impl From<&L4Protocol> for &'static str {
	fn from(protocol: &L4Protocol) -> Self {
		match protocol {
			L4Protocol::IPPROTO_IP => "IP",
			L4Protocol::IPPROTO_ICMP => "ICMP",
			L4Protocol::IPPROTO_IGMP => "IGMP",
			L4Protocol::IPPROTO_IPIP => "IPIP",
			L4Protocol::IPPROTO_TCP => "TCP",
			L4Protocol::IPPROTO_EGP => "EGP",
			L4Protocol::IPPROTO_PUP => "PUP",
			L4Protocol::IPPROTO_UDP => "UDP",
			L4Protocol::IPPROTO_IDP => "IDP",
			L4Protocol::IPPROTO_TP => "TP",
			L4Protocol::IPPROTO_DCCP => "DCCP",
			L4Protocol::IPPROTO_IPV6 => "IPv6",
			L4Protocol::IPPROTO_RSVP => "RSVP",
			L4Protocol::IPPROTO_GRE => "GRE",
			L4Protocol::IPPROTO_ESP => "ESP",
			L4Protocol::IPPROTO_AH => "AH",
			L4Protocol::IPPROTO_MTP => "MTP",
			L4Protocol::IPPROTO_BEETPH => "BEETPH",
			L4Protocol::IPPROTO_ENCAP => "ENCAP",
			L4Protocol::IPPROTO_PIM => "PIM",
			L4Protocol::IPPROTO_COMP => "COMP",
			L4Protocol::IPPROTO_L2TP => "L2TP",
			L4Protocol::IPPROTO_SCTP => "SCTP",
			L4Protocol::IPPROTO_UDPLITE => "UDPLITE",
			L4Protocol::IPPROTO_MPLS => "MPLS",
			L4Protocol::IPPROTO_ETHERNET => "ETHERNET",
			L4Protocol::IPPROTO_RAW => "RAW",
			L4Protocol::IPPROTO_MPTCP => "MPTCP",
		}
	}
}
