use core::fmt;
use num_enum::{FromPrimitive, IntoPrimitive};
use serde::Serialize;

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum ProtocolFamily {
	/// Unspecified.
	PF_UNSPEC = 0,
	/// Local to host (pipes and file-domain).
	PF_LOCAL = 1,
	// /// POSIX name for PF_LOCAL.
	// PF_UNIX = 1,
	// /// Another non-standard name for PF_LOCAL.
	// PF_FILE = 1,
	/// IP protocol family.
	PF_INET = 2,
	/// Amateur Radio AX.25.
	PF_AX25 = 3,
	/// Novell Internet Protocol.
	PF_IPX = 4,
	/// Appletalk DDP.
	PF_APPLETALK = 5,
	/// Amateur radio NetROM.
	PF_NETROM = 6,
	/// Multiprotocol bridge.
	PF_BRIDGE = 7,
	/// ATM PVCs.
	PF_ATMPVC = 8,
	/// Reserved for X.25 project.
	PF_X25 = 9,
	/// IP version 6.
	PF_INET6 = 10,
	/// Amateur Radio X.25 PLP.
	PF_ROSE = 11,
	/// Reserved for DECnet project.
	PF_DECNET = 12,
	/// Reserved for 802.2LLC project.
	PF_NETBEUI = 13,
	/// Security callback pseudo AF.
	PF_SECURITY = 14,
	/// PF_KEY key management API.
	PF_KEY = 15,
	PF_NETLINK = 16,
	// /// Alias to emulate 4.4BSD.
	//     PF_ROUTE = PF_NETLINK,
	/// Packet family.
	PF_PACKET = 17,
	/// Ash.
	PF_ASH = 18,
	/// Acorn Econet.
	PF_ECONET = 19,
	/// ATM SVCs.
	PF_ATMSVC = 20,
	/// RDS sockets.
	PF_RDS = 21,
	/// Linux SNA Project
	PF_SNA = 22,
	/// IRDA sockets.
	PF_IRDA = 23,
	/// PPPoX sockets.
	PF_PPPOX = 24,
	/// Wanpipe API sockets.
	PF_WANPIPE = 25,
	/// Linux LLC.
	PF_LLC = 26,
	/// Native InfiniBand address.
	PF_IB = 27,
	/// MPLS.
	PF_MPLS = 28,
	/// Controller Area Network.
	PF_CAN = 29,
	/// TIPC sockets.
	PF_TIPC = 30,
	/// Bluetooth sockets.
	PF_BLUETOOTH = 31,
	/// IUCV sockets.
	PF_IUCV = 32,
	/// RxRPC sockets.
	PF_RXRPC = 33,
	/// mISDN sockets.
	PF_ISDN = 34,
	/// Phonet sockets.
	PF_PHONET = 35,
	/// IEEE 802.15.4 sockets.
	PF_IEEE802154 = 36,
	/// CAIF sockets.
	PF_CAIF = 37,
	/// Algorithm sockets.
	PF_ALG = 38,
	/// NFC sockets.
	PF_NFC = 39,
	/// vSockets.
	PF_VSOCK = 40,
	/// Kernel Connection Multiplexor.
	PF_KCM = 41,
	/// Qualcomm IPC Router.
	PF_QIPCRTR = 42,
	/// SMC sockets.
	PF_SMC = 43,
	/// XDP sockets.
	PF_XDP = 44,
	/// Management component transport protocol.
	PF_MCTP = 45,
	/// For now.
	PF_MAX = 46,
	#[num_enum(catch_all)]
	PF_Reversed(u16),
}

impl From<&ProtocolFamily> for &'static str {
	fn from(protocol: &ProtocolFamily) -> Self {
		match protocol {
			ProtocolFamily::PF_UNSPEC => "Unspecified",
			ProtocolFamily::PF_LOCAL => "Local to host (pipes and file-domain)",
			ProtocolFamily::PF_INET => "IPv4",
			ProtocolFamily::PF_AX25 => "Amateur Radio AX.25",
			ProtocolFamily::PF_IPX => "Novell Internet Protocol",
			ProtocolFamily::PF_APPLETALK => "Appletalk DDP",
			ProtocolFamily::PF_NETROM => "Amateur radio NetROM",
			ProtocolFamily::PF_BRIDGE => "Multiprotocol bridge",
			ProtocolFamily::PF_ATMPVC => "ATM PVCs",
			ProtocolFamily::PF_X25 => "Reserved for X.25 project",
			ProtocolFamily::PF_INET6 => "IPv6",
			ProtocolFamily::PF_ROSE => "Amateur Radio X.25 PLP",
			ProtocolFamily::PF_DECNET => "Reserved for DECnet project",
			ProtocolFamily::PF_NETBEUI => "Reserved for 802.2LLC project",
			ProtocolFamily::PF_SECURITY => "Security callback pseudo AF",
			ProtocolFamily::PF_KEY => "PF_KEY key management API",
			ProtocolFamily::PF_NETLINK => "Netlink",
			ProtocolFamily::PF_PACKET => "Packet family",
			ProtocolFamily::PF_ASH => "Ash",
			ProtocolFamily::PF_ECONET => "Acorn Econet",
			ProtocolFamily::PF_ATMSVC => "ATM SVCs",
			ProtocolFamily::PF_RDS => "RDS sockets",
			ProtocolFamily::PF_SNA => "Linux SNA Project",
			ProtocolFamily::PF_IRDA => "IRDA sockets",
			ProtocolFamily::PF_PPPOX => "PPPoX sockets",
			ProtocolFamily::PF_WANPIPE => "Wanpipe API sockets",
			ProtocolFamily::PF_LLC => "Linux LLC",
			ProtocolFamily::PF_IB => "Native InfiniBand address",
			ProtocolFamily::PF_MPLS => "MPLS",
			ProtocolFamily::PF_CAN => "Controller Area Network",
			ProtocolFamily::PF_TIPC => "TIPC sockets",
			ProtocolFamily::PF_BLUETOOTH => "Bluetooth sockets",
			ProtocolFamily::PF_IUCV => "IUCV sockets",
			ProtocolFamily::PF_RXRPC => "RxRPC sockets",
			ProtocolFamily::PF_ISDN => "mISDN sockets",
			ProtocolFamily::PF_PHONET => "Phonet sockets",
			ProtocolFamily::PF_IEEE802154 => "IEEE 802.15.4 sockets",
			ProtocolFamily::PF_CAIF => "CAIF sockets",
			ProtocolFamily::PF_ALG => "Algorithm sockets",
			ProtocolFamily::PF_NFC => "NFC sockets",
			ProtocolFamily::PF_VSOCK => "vSockets",
			ProtocolFamily::PF_KCM => "Kernel Connection Multiplexor",
			ProtocolFamily::PF_QIPCRTR => "Qualcomm IPC Router",
			ProtocolFamily::PF_SMC => "SMC sockets",
			ProtocolFamily::PF_XDP => "XDP sockets",
			ProtocolFamily::PF_MCTP => "Management component transport protocol",
			ProtocolFamily::PF_MAX => "PF_MAX For now..",
			ProtocolFamily::PF_Reversed(_) => "Reversed",
		}
	}
}

impl fmt::Display for ProtocolFamily {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(self.into())
	}
}

/// Standard well-defined IP protocols.
#[derive(FromPrimitive, IntoPrimitive, Copy, Clone, PartialEq, Hash, Eq, Serialize, Debug)]
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
	#[num_enum(catch_all)]
	IPPROTO_MAX(u16),
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
			L4Protocol::IPPROTO_MAX(_) => "MAX",
		}
	}
}
