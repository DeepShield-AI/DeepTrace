use num_enum::{FromPrimitive, IntoPrimitive};

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

#[cfg(feature = "user")]
impl std::fmt::Display for SaFamily {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::AF_UNSPEC => write!(f, "Unspecified"),
			Self::AF_UNIX => write!(f, "Unix domain sockets"),
			Self::AF_INET => write!(f, "IPv4"),
			Self::AF_AX25 => write!(f, "Amateur Radio AX.25"),
			Self::AF_IPX => write!(f, "Novell Internet Protocol"),
			Self::AF_APPLETALK => write!(f, "Appletalk DDP"),
			Self::AF_NETROM => write!(f, "Amateur radio NetROM"),
			Self::AF_BRIDGE => write!(f, "Multiprotocol bridge"),
			Self::AF_ATMPVC => write!(f, "ATM PVCs"),
			Self::AF_X25 => write!(f, "Reserved for X.25 project"),
			Self::AF_INET6 => write!(f, "IPv6"),
			Self::AF_ROSE => write!(f, "Amateur Radio X.25 PLP"),
			Self::AF_DECnet => write!(f, "Reserved for DECnet project"),
			Self::AF_NETBEUI => write!(f, "Reserved for 802.2LLC project"),
			Self::AF_SECURITY => write!(f, "Security callback pseudo AF"),
			Self::AF_KEY => write!(f, "AF_KEY key management API"),
			Self::AF_NETLINK => write!(f, "Netlink"),
			Self::AF_PACKET => write!(f, "Packet family"),
			Self::AF_ASH => write!(f, "Ash"),
			Self::AF_ECONET => write!(f, "Acorn Econet"),
			Self::AF_ATMSVC => write!(f, "ATM SVCs"),
			Self::AF_RDS => write!(f, "RDS sockets"),
			Self::AF_SNA => write!(f, "Linux SNA Project"),
			Self::AF_IRDA => write!(f, "IRDA sockets"),
			Self::AF_PPPOX => write!(f, "PPPoX sockets"),
			Self::AF_WANPIPE => write!(f, "Wanpipe API sockets"),
			Self::AF_LLC => write!(f, "Linux LLC"),
			Self::AF_IB => write!(f, "Native InfiniBand address"),
			Self::AF_MPLS => write!(f, "MPLS"),
			Self::AF_CAN => write!(f, "Controller Area Network"),
			Self::AF_TIPC => write!(f, "TIPC sockets"),
			Self::AF_BLUETOOTH => write!(f, "Bluetooth sockets"),
			Self::AF_IUCV => write!(f, "IUCV sockets"),
			Self::AF_RXRPC => write!(f, "RxRPC sockets"),
			Self::AF_ISDN => write!(f, "mISDN sockets"),
			Self::AF_PHONET => write!(f, "Phonet sockets"),
			Self::AF_IEEE802154 => write!(f, "IEEE 802.15.4 sockets"),
			Self::AF_CAIF => write!(f, "CAIF sockets"),
			Self::AF_ALG => write!(f, "Algorithm sockets"),
			Self::AF_NFC => write!(f, "NFC sockets"),
			Self::AF_VSOCK => write!(f, "vSockets"),
			Self::AF_KCM => write!(f, "Kernel Connection Multiplexor"),
			Self::AF_QIPCRTR => write!(f, "Qualcomm IPC Router"),
			Self::AF_SMC => write!(f, "SMC sockets"),
			Self::AF_XDP => write!(f, "XDP sockets"),
			Self::AF_MCTP => write!(f, "Management component transport protocol"),
			Self::AF_MAX => write!(f, "AF_MAX For now."),
			Self::AF_Reversed(bad) => write!(f, "Reversed {}", bad),
		}
	}
}

/// Standard well-defined IP protocols.
#[cfg_attr(feature = "user", derive(serde::Serialize, Hash, Eq))]
#[derive(Clone, Copy, PartialEq)]
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

#[cfg(feature = "user")]
impl std::fmt::Display for L4Protocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::IPPROTO_IP => write!(f, "IP"),
			Self::IPPROTO_ICMP => write!(f, "ICMP"),
			Self::IPPROTO_IGMP => write!(f, "IGMP"),
			Self::IPPROTO_IPIP => write!(f, "IPIP"),
			Self::IPPROTO_TCP => write!(f, "TCP"),
			Self::IPPROTO_EGP => write!(f, "EGP"),
			Self::IPPROTO_PUP => write!(f, "PUP"),
			Self::IPPROTO_UDP => write!(f, "UDP"),
			Self::IPPROTO_IDP => write!(f, "IDP"),
			Self::IPPROTO_TP => write!(f, "TP"),
			Self::IPPROTO_DCCP => write!(f, "DCCP"),
			Self::IPPROTO_IPV6 => write!(f, "IPv6"),
			Self::IPPROTO_RSVP => write!(f, "RSVP"),
			Self::IPPROTO_GRE => write!(f, "GRE"),
			Self::IPPROTO_ESP => write!(f, "ESP"),
			Self::IPPROTO_AH => write!(f, "AH"),
			Self::IPPROTO_MTP => write!(f, "MTP"),
			Self::IPPROTO_BEETPH => write!(f, "BEETPH"),
			Self::IPPROTO_ENCAP => write!(f, "ENCAP"),
			Self::IPPROTO_PIM => write!(f, "PIM"),
			Self::IPPROTO_COMP => write!(f, "COMP"),
			Self::IPPROTO_L2TP => write!(f, "L2TP"),
			Self::IPPROTO_SCTP => write!(f, "SCTP"),
			Self::IPPROTO_UDPLITE => write!(f, "UDPLITE"),
			Self::IPPROTO_MPLS => write!(f, "MPLS"),
			Self::IPPROTO_ETHERNET => write!(f, "ETHERNET"),
			Self::IPPROTO_RAW => write!(f, "RAW"),
			Self::IPPROTO_MPTCP => write!(f, "MPTCP"),
		}
	}
}
