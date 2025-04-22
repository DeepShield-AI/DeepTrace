use crate::{
	structs::{Args, InferInfo, SocketInfo},
	vmlinux::{fdtable, file, files_struct, socket, task_struct, tcp_sock},
};
use aya_ebpf::{
	macros::map,
	maps::{HashMap, PerCpuArray, PerfEventArray},
};
use mercury_common::{
	consts::{MAX_INFER_PAYLOAD_SIZE, MAX_PID_NUMBER},
	structs::Data,
};

/// Filter the trigger of system call hooks by pid generated at user space.
#[map(name = "pids")]
pub(crate) static mut PIDS: HashMap<u32, u32> = HashMap::with_max_entries(MAX_PID_NUMBER, 0);

/// For large structures, use `PerCpuArray` to avoid exceed the stack memory
#[map(name = "task_struct")]
pub(crate) static mut TASK_STRUCT: PerCpuArray<task_struct> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "files_struct")]
pub(crate) static mut FILES_STRUCT: PerCpuArray<files_struct> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "fdtable")]
pub static mut FD_TABLE: PerCpuArray<fdtable> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "file")]
pub(crate) static mut FILE: PerCpuArray<file> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "socket")]
pub static mut SOCKET: PerCpuArray<socket> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "tcp_sock")]
pub(crate) static mut TCP_SOCK: PerCpuArray<tcp_sock> = PerCpuArray::with_max_entries(1, 0);

/// Storage params when enter syscalls.
#[map(name = "ingress")]
pub(crate) static mut INGRESS: HashMap<u64, Args> = HashMap::with_max_entries((1 << 10) * 10, 0);
#[map(name = "egress")]
pub(crate) static mut EGRESS: HashMap<u64, Args> = HashMap::with_max_entries((1 << 10) * 10, 0);

/// Storage socket info.
#[map(name = "socket_info")]
pub(crate) static mut SOCKET_INFO: HashMap<u64, SocketInfo> =
	HashMap::with_max_entries((1 << 10) * 10, 0);
/// Infer protocol.
#[map(name = "protocol")]
pub(crate) static mut INFER: PerCpuArray<InferInfo> = PerCpuArray::with_max_entries(1, 0);
// TODO: change this size
#[map(name = "infer_buffer")]
pub(crate) static mut INFER_BUFFER: PerCpuArray<[u8; MAX_INFER_PAYLOAD_SIZE as usize * 128]> =
	PerCpuArray::with_max_entries(1, 0);
// #[map(name = "message")]
// pub(crate) static mut MESSAGE: RingBuf =
// 	RingBuf::with_byte_size(size_of::<Data>() as u32 * (1 << 12), 0);
/// Struct for eBPF kernel data transform to user space.
#[map(name = "data")]
pub(crate) static mut DATA: PerCpuArray<Data> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "events")]
pub(crate) static mut EVENTS: PerfEventArray<Data> = PerfEventArray::new(0);
