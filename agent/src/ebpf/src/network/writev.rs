use crate::{
	network::process::{try_enter, try_exit},
	structs::Args,
	utils::{is_filtered_pid, write_seq},
	vmlinux::iovec,
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::bpf_ktime_get_ns,
	macros::tracepoint,
	programs::TracePointContext,
};
use mercury_common::{SyscallName, SyscallType};

/// `name`: sys_enter_writev
/// `ID`: 693
///
///     field:unsigned long fd; offset:16;      size:8; signed:0;
///     field:const struct iovec * vec; offset:24;      size:8; signed:0;
///     field:unsigned long vlen;       offset:32;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_writev")]
fn sys_enter_writev(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let timestamp = unsafe { bpf_ktime_get_ns() };
	let fd = match unsafe { ctx.read_at::<c_ulong>(16) } {
		Ok(fd) => fd as u32,
		Err(_) => return 0,
	};
	let vec: *mut iovec = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(vec) => vec as *mut iovec,
		Err(_) => return 0,
	};
	let vlen = match unsafe { ctx.read_at::<c_ulong>(32) } {
		Ok(vlen) => vlen,
		Err(_) => return 0,
	};
	let seq = match write_seq(fd) {
		Ok(seq) => seq,
		Err(_) => return 0,
	};
	let args = Args::vectored(fd, seq, vec, vlen, timestamp);
	try_enter(ctx, args, SyscallType::Egress).unwrap_or_else(|ret| ret)
}
/// name: sys_exit_writev
/// ID: 690
///
///     field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_writev")]
fn sys_exit_writev(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let ret = match unsafe { ctx.read_at::<c_long>(16) } {
		Ok(ret) => ret,
		Err(_) => return 0,
	};
	try_exit(ctx, ret, SyscallName::WriteV, SyscallType::Egress).unwrap_or_else(|ret| ret)
}
