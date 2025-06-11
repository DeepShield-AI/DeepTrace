use crate::vmlinux::{ipc_namespace, net, task_struct, uts_namespace};
use aya_ebpf::{
	macros::map,
	maps::{PerCpuArray, PerfEventByteArray},
};

#[map(name = "events")]
pub(crate) static mut OUTPUT: PerfEventByteArray = PerfEventByteArray::new(0);

#[map(name = "edges")]
pub(crate) static mut EDGE: PerfEventByteArray = PerfEventByteArray::new(0);

#[map(name = "net")]
pub(crate) static mut NET: PerCpuArray<net> = PerCpuArray::with_max_entries(1, 0);

#[map(name = "uts_namespace")]
pub(crate) static mut UTS_NAMESPACE: PerCpuArray<uts_namespace> =
	PerCpuArray::with_max_entries(1, 0);

#[map(name = "ipc_namespace")]
pub(crate) static mut IPC_NAMESPACE: PerCpuArray<ipc_namespace> =
	PerCpuArray::with_max_entries(1, 0);

#[map(name = "task_struct")]
pub(crate) static mut TASK_STRUCT: PerCpuArray<task_struct> = PerCpuArray::with_max_entries(1, 0);
