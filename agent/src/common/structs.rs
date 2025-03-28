use super::{protocol::*, MAX_PAYLOAD_SIZE, TASK_CMD_LEN};
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SyscallName {
	Read,
	RecvMsg,
	RecvMMsg,
	ReadV,
	RecvFrom,
	Write,
	SendMsg,
	SendMMsg,
	SendTo,
	WriteV,
	Unknown,
}

#[cfg(feature = "user")]
impl std::fmt::Display for SyscallName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SyscallName::Read => f.write_str("Read"),
			SyscallName::RecvMsg => f.write_str("RecvMsg"),
			SyscallName::RecvMMsg => f.write_str("RecvMMsg"),
			SyscallName::ReadV => f.write_str("ReadV"),
			SyscallName::RecvFrom => f.write_str("RecvFrom"),
			SyscallName::Write => f.write_str("Write"),
			SyscallName::SendMsg => f.write_str("SendMsg"),
			SyscallName::SendMMsg => f.write_str("SendMMsg"),
			SyscallName::SendTo => f.write_str("SendTo"),
			SyscallName::WriteV => f.write_str("WriteV"),
			SyscallName::Unknown => f.write_str("Unknown"),
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SyscallType {
	Ingress,
	Egress,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Quintuple {
	pub src_addr: u32,
	pub dst_addr: u32,
	pub src_port: u16,
	pub dst_port: u16,
	/// protocol families
	pub skc_family: u16,
}
impl Quintuple {
	pub fn new(
		src_addr: u32,
		dst_addr: u32,
		src_port: u16,
		dst_port: u16,
		skc_family: u16,
	) -> Quintuple {
		Self { src_addr, dst_addr, src_port, dst_port, skc_family }
	}

	pub fn get_protocol_name(&self) -> &'static str {
		match self.skc_family {
			PF_UNSPEC => "Unspecified",
			PF_LOCAL => "Local to host (pipes and file-domain)",
			PF_INET => "IP protocol family",
			PF_AX25 => "Amateur Radio AX.25",
			PF_IPX => "Novell Internet Protocol",
			PF_APPLETALK => "Appletalk DDP",
			PF_NETROM => "Amateur radio NetROM",
			PF_BRIDGE => "Multiprotocol bridge",
			PF_ATMPVC => "ATM PVCs",
			PF_X25 => "Reserved for X.25 project",
			PF_INET6 => "IP version 6",
			PF_ROSE => "Amateur Radio X.25 PLP",
			PF_DECNET => "Reserved for DECnet project",
			PF_NETBEUI => "Reserved for 802.2LLC project",
			PF_SECURITY => "Security callback pseudo AF",
			PF_KEY => "PF_KEY key management API",
			PF_NETLINK => "Netlink",
			PF_PACKET => "Packet family",
			PF_ASH => "Ash",
			PF_ECONET => "Acorn Econet",
			PF_ATMSVC => "ATM SVCs",
			PF_RDS => "RDS sockets",
			PF_SNA => "Linux SNA Project",
			PF_IRDA => "IRDA sockets",
			PF_PPPOX => "PPPoX sockets",
			PF_WANPIPE => "Wanpipe API sockets",
			PF_LLC => "Linux LLC",
			PF_IB => "Native InfiniBand address",
			PF_MPLS => "MPLS",
			PF_CAN => "Controller Area Network",
			PF_TIPC => "TIPC sockets",
			PF_BLUETOOTH => "Bluetooth sockets",
			PF_IUCV => "IUCV sockets",
			PF_RXRPC => "RxRPC sockets",
			PF_ISDN => "mISDN sockets",
			PF_PHONET => "Phonet sockets",
			PF_IEEE802154 => "IEEE 802.15.4 sockets",
			PF_CAIF => "CAIF sockets",
			PF_ALG => "Algorithm sockets",
			PF_NFC => "NFC sockets",
			PF_VSOCK => "vSockets",
			PF_KCM => "Kernel Connection Multiplexor",
			PF_QIPCRTR => "Qualcomm IPC Router",
			PF_SMC => "SMC sockets",
			PF_XDP => "XDP sockets",
			PF_MCTP => "Management component transport protocol",
			PF_MAX => "For now..",
			_ => "unknown",
		}
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for Quintuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"skc_family: {}, saddr: {}, daddr: {}, sport: {}, dport: {}",
			self.get_protocol_name(),
			std::net::Ipv4Addr::from(self.src_addr.to_be_bytes()),
			std::net::Ipv4Addr::from(self.dst_addr.to_be_bytes()),
			self.src_port,
			self.dst_port,
		))
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Data {
	pub tgid: u32,
	pub pid: u32,
	pub enter_seq: u32,
	pub exit_seq: u32,
	pub timestamp_ns: u64,
	pub len: u32,
	pub syscall: SyscallName,
	pub direction: SyscallType,
	pub quintuple: Quintuple,
	pub comm: [u8; TASK_CMD_LEN],
	// pub pre_len: u32,
	// pub pre_payload: [u8; MAX_PRE_PAYLOAD_SIZE as usize],
	pub buf: [u8; MAX_PAYLOAD_SIZE as usize],
}

impl Data {
	pub fn clear(&mut self) {
		self.tgid = 0;
		self.pid = 0;
		self.enter_seq = 0;
		self.exit_seq = 0;
		self.timestamp_ns = 0;
		self.len = 0;
		self.syscall = SyscallName::Unknown;
		self.quintuple = Quintuple::new(0, 0, 0, 0, 0);
		self.comm = [0; TASK_CMD_LEN];
		// self.pre_len = 0;
		// self.pre_payload = [0; MAX_PRE_PAYLOAD_SIZE as usize];
		self.buf = [0; MAX_PAYLOAD_SIZE as usize];
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for Data {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
            f,
            "tgid: {:?}, Time: {:?}, Command: {:?}, Syscall: {}, Direction: {:?}, Length: {}, Data: {:?}",
            self.tgid,
            self.timestamp_ns,
            // Convert 'cmd' and 'buf' fields to strings for display.
            // 'String::from_utf8_lossy' will replace invalid UTF-8 sequences with U+FFFD REPLACEMENT CHARACTER.
            String::from_utf8_lossy(&self.comm),
            self.syscall,
            self.direction,
            self.len,
            String::from_utf8_lossy(&self.buf[..self.len as usize]),
        )
	}
}

#[cfg(feature = "user")]
impl Data {
	pub fn buffer(&self) -> Vec<u8> {
		self.buf[..self.len as usize].to_vec()
		// self.pre_payload[..self.pre_len as usize]
		// 	.iter()
		// 	.chain(self.buf[..self.len as usize].iter())
		// 	.copied()
		// 	.collect()
	}
}
