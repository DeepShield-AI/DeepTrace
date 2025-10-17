use crate::types::{Args, SocketInfo};
use aya_ebpf::{
	macros::map,
	maps::{HashMap, PerfEventByteArray},
};
use observ_trace_common::constants::MAX_PID_NUMBERS;

/// Filter the trigger of system call hooks by pid generated at user space.
#[map(name = "PIDS")]
pub(crate) static mut PIDS: HashMap<u32, u32> = HashMap::with_max_entries(MAX_PID_NUMBERS, 0);
/// Storage params when enter syscalls.
#[map(name = "ingress")]
pub(crate) static mut INGRESS: HashMap<u64, Args> = HashMap::with_max_entries(1 << 10, 0);
#[map(name = "egress")]
pub(crate) static mut EGRESS: HashMap<u64, Args> = HashMap::with_max_entries(1 << 10, 0);

/// Storage socket info.
#[map(name = "socket_info")]
pub(crate) static mut SOCKET_INFO: HashMap<u64, SocketInfo> = HashMap::with_max_entries(1 << 10, 0);

#[map(name = "EVENTS")]
pub(crate) static mut EVENTS: PerfEventByteArray = PerfEventByteArray::new(0);
