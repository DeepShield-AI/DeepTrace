use crate::{
	maps::{IPC_NAMESPACE, NET, OUTPUT, UTS_NAMESPACE},
	vmlinux::{ipc_namespace, net, task_struct, uts_namespace},
};
use aya_ebpf::{
	cty::c_int,
	helpers::{bpf_probe_read as bpf_help_read, gen::bpf_probe_read, r#gen::bpf_get_current_task},
	macros::lsm,
	programs::LsmContext,
};
use aya_log_ebpf::info;
use provenance_common::{MetaData, Task, TaskPoint, Version};

#[lsm(hook = "task_free")]
pub fn task_free(ctx: LsmContext) -> i32 {
	match unsafe { try_task_free(ctx) } {
		Ok(ret) => ret,
		Err(ret) => ret,
	}
}

unsafe fn try_task_free(ctx: LsmContext) -> Result<i32, i32> {
	let task: *const task_struct = ctx.arg(0);
	let ns_proxy = bpf_help_read(({ &*task }).nsproxy).map_err(|_| 0)?;
	let uts_ns = UTS_NAMESPACE.get_ptr_mut(0).ok_or(0_i32)?;
	if bpf_probe_read(
		uts_ns as *mut _,
		size_of::<uts_namespace>() as u32,
		ns_proxy.uts_ns as *const _,
	) != 0
	{
		return Ok(0);
	}
	let ipc_ns = IPC_NAMESPACE.get_ptr_mut(0).ok_or(0_i32)?;
	if bpf_probe_read(
		ipc_ns as *mut _,
		size_of::<ipc_namespace>() as u32,
		ns_proxy.ipc_ns as *const _,
	) != 0
	{
		return Ok(0);
	}
	let mnt_ns = bpf_help_read(ns_proxy.mnt_ns).map_err(|_| 0)?;
	let pid_ns = bpf_help_read(ns_proxy.pid_ns_for_children).map_err(|_| 0)?;
	let net_ns = NET.get_ptr_mut(0).ok_or(0_i32)?;
	if bpf_probe_read(net_ns as *mut _, size_of::<net>() as u32, ns_proxy.net_ns as *const _) != 0 {
		return Ok(0);
	}
	let cgroup_ns = bpf_help_read(ns_proxy.cgroup_ns).map_err(|_| 0)?;

	let task = TaskPoint {
		meta: MetaData { identifier: 0, epoch: 0, jiffies: 0, taint: 0 },
		version: Version { name: 0, prev: 0 },
		data: Task {
			pid: (&*task).pid as u32,
			utime: (&*task).utime,
			stime: (&*task).stime,
			utsns: (&*uts_ns).ns.inum,
			ipcns: (&*ipc_ns).ns.inum,
			mntns: mnt_ns.ns.inum,
			pidns: pid_ns.ns.inum,
			netns: (&*net_ns).ns.inum,
			cgroupns: cgroup_ns.ns.inum,
		},
	};
	OUTPUT.output(&ctx, task.encode(), 0);
	// info!(&ctx, "new pid {}", pid);

	Ok(0)
}
