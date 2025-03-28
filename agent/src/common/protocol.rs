/// Unspecified.
pub const PF_UNSPEC: u16 = 0;
/// Local to host (pipes and file-domain).
pub const PF_LOCAL: u16 = 1;
/// POSIX name for PF_LOCAL.
pub const PF_UNIX: u16 = PF_LOCAL;
/// Another non-standard name for PF_LOCAL.
pub const PF_FILE: u16 = PF_LOCAL;
/// IP protocol family.
pub const PF_INET: u16 = 2;
/// Amateur Radio AX.25.
pub const PF_AX25: u16 = 3;
/// Novell Internet Protocol.
pub const PF_IPX: u16 = 4;
/// Appletalk DDP.
pub const PF_APPLETALK: u16 = 5;
/// Amateur radio NetROM.
pub const PF_NETROM: u16 = 6;
/// Multiprotocol bridge.
pub const PF_BRIDGE: u16 = 7;
/// ATM PVCs.
pub const PF_ATMPVC: u16 = 8;
/// Reserved for X.25 project.
pub const PF_X25: u16 = 9;
/// IP version 6.
pub const PF_INET6: u16 = 10;
/// Amateur Radio X.25 PLP.
pub const PF_ROSE: u16 = 11;
/// Reserved for DECnet project.
pub const PF_DECNET: u16 = 12;
/// Reserved for 802.2LLC project.
pub const PF_NETBEUI: u16 = 13;
/// Security callback pseudo AF.
pub const PF_SECURITY: u16 = 14;
/// PF_KEY key management API.
pub const PF_KEY: u16 = 15;
pub const PF_NETLINK: u16 = 16;
/// Alias to emulate 4.4BSD.
pub const PF_ROUTE: u16 = PF_NETLINK;
/// Packet family.
pub const PF_PACKET: u16 = 17;
/// Ash.
pub const PF_ASH: u16 = 18;
/// Acorn Econet.
pub const PF_ECONET: u16 = 19;
/// ATM SVCs.
pub const PF_ATMSVC: u16 = 20;
/// RDS sockets.
pub const PF_RDS: u16 = 21;
/// Linux SNA Project
pub const PF_SNA: u16 = 22; /*  */
/// IRDA sockets.
pub const PF_IRDA: u16 = 23;
/// PPPoX sockets.
pub const PF_PPPOX: u16 = 24;
/// Wanpipe API sockets.
pub const PF_WANPIPE: u16 = 25;
/// Linux LLC.
pub const PF_LLC: u16 = 26;
/// Native InfiniBand address.
pub const PF_IB: u16 = 27;
/// MPLS.
pub const PF_MPLS: u16 = 28;
/// Controller Area Network.
pub const PF_CAN: u16 = 29;
/// TIPC sockets.
pub const PF_TIPC: u16 = 30;
/// Bluetooth sockets.
pub const PF_BLUETOOTH: u16 = 31;
/// IUCV sockets.
pub const PF_IUCV: u16 = 32;
/// RxRPC sockets.
pub const PF_RXRPC: u16 = 33;
/// mISDN sockets.
pub const PF_ISDN: u16 = 34;
/// Phonet sockets.
pub const PF_PHONET: u16 = 35;
/// IEEE 802.15.4 sockets.
pub const PF_IEEE802154: u16 = 36;
/// CAIF sockets.
pub const PF_CAIF: u16 = 37;
/// Algorithm sockets.
pub const PF_ALG: u16 = 38;
/// NFC sockets.
pub const PF_NFC: u16 = 39;
/// vSockets.
pub const PF_VSOCK: u16 = 40;
/// Kernel Connection Multiplexor.
pub const PF_KCM: u16 = 41;
/// Qualcomm IPC Router.
pub const PF_QIPCRTR: u16 = 42;
/// SMC sockets.
pub const PF_SMC: u16 = 43;
/// XDP sockets.
pub const PF_XDP: u16 = 44;
/// Management component transport protocol.
pub const PF_MCTP: u16 = 45;
/// For now.
pub const PF_MAX: u16 = 46;
