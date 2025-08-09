use crate::{
	constants::{MAX_INFER_PAYLOAD_SIZE, MAX_PID_NUMBER},
	structs::{Args, InferInfo, SocketInfo},
	vmlinux::{fdtable, file, files_struct, socket, task_struct, tcp_sock},
};
use aya_ebpf::{
	macros::map,
	maps::{HashMap, PerCpuArray, PerfEventArray},
};
use trace_common::structs::Data;
//metric

pub const MAX_SOFTIRQS: usize = 10;

/// Filter the trigger of system call hooks by pid generated at user space.
/// // pids
#[map(name = "pids")]
pub(crate) static mut PIDS: HashMap<u32, u32> = HashMap::with_max_entries(1024, 0);
//io metric map
// 于存储插入时间的Map，仍然按设备区分读写
#[map(name = "INSERT_READ_TIMESTAMP")]
pub(crate) static mut INSERT_READ_TIMESTAMP: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "INSERT_WRITE_TIMESTAMP")]
pub(crate) static mut INSERT_WRITE_TIMESTAMP: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);
#[map(name = "READ_IO_COUNT")]
pub(crate) static mut READ_IO_COUNT: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "WRITE_IO_COUNT")]
pub(crate) static mut WRITE_IO_COUNT: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);
#[map(name = "READ_IOPS")]
pub(crate) static mut READ_IOPS: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "WRITE_IOPS")]
pub(crate) static mut WRITE_IOPS: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "READ_MERGE")]
pub(crate) static mut READ_MERGE: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "WRITE_MERGE")]
pub(crate) static mut WRITE_MERGE: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "READ_BPS")]
pub(crate) static mut READ_BPS: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "WRITE_BPS")]
pub(crate) static mut WRITE_BPS: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "READ_ISSUE_NSEC")]
pub(crate) static mut READ_ISSUE_NSEC: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "WRITE_ISSUE_NSEC")]
pub(crate) static mut WRITE_ISSUE_NSEC: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "READ_NSEC")]
pub(crate) static mut READ_NSEC: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "WRITE_NSEC")]
pub(crate) static mut WRITE_NSEC: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);
// cpu metric map
#[map(name = "SOFTIRQ_TIMESTAMPS")]
pub(crate) static mut SOFTIRQ_TIMESTAMPS: PerCpuArray<u64> = PerCpuArray::with_max_entries(MAX_SOFTIRQS as u32, 0);
#[map(name = "KSOFTIRQD_DELAY")]
pub(crate) static mut KSOFTIRQD_DELAY: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "CPU_MIGRATIONS")]
pub(crate) static mut CPU_MIGRATIONS: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

#[map(name = "CONTEXT_SWITCHES")]
pub(crate) static mut CONTEXT_SWITCHES: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);

//mem metric map
#[map(name = "WAKEUP_KSWAPD")]
pub(crate) static mut WAKEUP_KSWAPD: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);
#[map(name = "PAGE_ALLOC_EXTFRAG")]
pub(crate) static mut PAGE_ALLOC_EXTFRAG: HashMap<u32, u64> = HashMap::with_max_entries(128, 0);


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
pub(crate) static mut INGRESS: HashMap<u64, Args> = HashMap::with_max_entries(1 << 10, 0);
#[map(name = "egress")]
pub(crate) static mut EGRESS: HashMap<u64, Args> = HashMap::with_max_entries(1 << 10, 0);

/// Storage socket info.
#[map(name = "socket_info")]
pub(crate) static mut SOCKET_INFO: HashMap<u64, SocketInfo> = HashMap::with_max_entries(1 << 10, 0);
/// Infer protocol.
#[map(name = "protocol")]
pub(crate) static mut INFER: PerCpuArray<InferInfo> = PerCpuArray::with_max_entries(1, 0);
// TODO: change this size
#[map(name = "infer_buffer")]
pub(crate) static mut INFER_BUFFER: PerCpuArray<[u8; MAX_INFER_PAYLOAD_SIZE as usize * 128]> =
	PerCpuArray::with_max_entries(1, 0);
/// Struct for eBPF kernel data transform to user space.
#[map(name = "data")]
pub(crate) static mut DATA: PerCpuArray<Data> = PerCpuArray::with_max_entries(1, 0);
#[map(name = "events")]
pub(crate) static mut EVENTS: PerfEventArray<Data> = PerfEventArray::new(0);
