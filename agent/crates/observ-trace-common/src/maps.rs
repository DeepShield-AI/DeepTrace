use crate::socket::SocketInfo;
use aya_ebpf::{macros::map, maps::HashMap};

pub const PIDS_MAP: &str = "PIDS";
pub const EVENT_MAP: &str = "EVENTS";

#[map(name = "socket_info")]
pub static mut SOCKET_INFO: HashMap<u64, SocketInfo> = HashMap::with_max_entries(1 << 10, 0);
