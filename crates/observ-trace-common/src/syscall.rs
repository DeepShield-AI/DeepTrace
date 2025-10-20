#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[repr(u8)]
pub enum Syscall {
	Read,
	ReadV,
	RecvFrom,
	RecvMsg,
	RecvMMsg,

	Write,
	WriteV,
	SendTo,
	SendMsg,
	SendMMsg,

	Unknown,
}

#[cfg(feature = "user")]
impl std::fmt::Display for Syscall {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Syscall::Read => write!(f, "read"),
			Syscall::ReadV => write!(f, "readv"),
			Syscall::RecvFrom => write!(f, "recvfrom"),
			Syscall::RecvMsg => write!(f, "recvmsg"),
			Syscall::RecvMMsg => write!(f, "recvmmsg"),

			Syscall::Write => write!(f, "write"),
			Syscall::WriteV => write!(f, "writev"),
			Syscall::SendTo => write!(f, "sendto"),
			Syscall::SendMsg => write!(f, "sendmsg"),
			Syscall::SendMMsg => write!(f, "sendmmsg"),

			Syscall::Unknown => write!(f, "unknown"),
		}
	}
}
