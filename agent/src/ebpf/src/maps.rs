use crate::{
	consts::MAX_PID_NUMBER,
	structs::{Args, PreKey, PrePayload},
	vmlinux::{file, files_struct, task_struct, tcp_sock},
};
use aya_ebpf::{
	macros::map,
	maps::{HashMap, LruHashMap, PerCpuArray, RingBuf},
};
use mercury_common::Data;

/// Filter the trigger of system call hooks by pid generated at user space.
#[map(name = "pids")]
pub static mut PIDS: HashMap<u32, u32> = HashMap::with_max_entries(MAX_PID_NUMBER, 0);

#[map(name = "task_struct")]
pub(crate) static mut TASK_STRUCT: PerCpuArray<task_struct> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "files_struct")]
pub(crate) static mut FILES_STRUCT: PerCpuArray<files_struct> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "file")]
pub(crate) static mut FILE: PerCpuArray<file> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "tcp_sock")]
pub(crate) static mut TCP_SOCK: PerCpuArray<tcp_sock> = PerCpuArray::with_max_entries(1, 0);

#[map(name = "ingress")]
pub(crate) static mut INGRESS: HashMap<u64, Args> = HashMap::with_max_entries((1 << 10) * 10, 0);
#[map(name = "egress")]
pub(crate) static mut EGRESS: HashMap<u64, Args> = HashMap::with_max_entries((1 << 10) * 10, 0);

#[map(name = "pre")]
pub(crate) static mut PRE_PAYLOAD: LruHashMap<PreKey, PrePayload> =
	LruHashMap::with_max_entries((1 << 10) * 10, 0);
/// Struct for eBPF kernel data transform to user space.
#[map(name = "message")]
pub(crate) static mut MESSAGE: RingBuf =
	RingBuf::with_byte_size(size_of::<Data>() as u32 * (1 << 12), 0);
