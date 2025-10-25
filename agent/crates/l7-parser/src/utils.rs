use observ_trace_common::{L7Protocol, maps::SOCKET_INFO};

pub(crate) fn check_protocol(key: u64, protocol: L7Protocol) -> bool {
	let Some(socket) = (unsafe { SOCKET_INFO.get(&key) }) else { return true };
	socket.l7protocol == protocol || socket.l7protocol == L7Protocol::Unknown
}
