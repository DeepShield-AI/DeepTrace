use crate::{
	network::process::{try_enter, try_exit},
	structs::Args,
	utils::{is_filtered_pid, read_seq},
	vmlinux::iovec,
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::bpf_ktime_get_ns,
	macros::tracepoint,
	programs::TracePointContext,
};
use mercury_common::{SyscallName, SyscallType};

/// `name`: sys_enter_readv `ID`: 693
///
///     field:unsigned long fd; offset:16;      size:8; signed:0;
///     field:const struct iovec * vec; offset:24;      size:8; signed:0;
///     field:unsigned long vlen;       offset:32;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_readv")]
fn sys_enter_readv(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	// info!(&ctx, "call readv");
	let timestamp = unsafe { bpf_ktime_get_ns() };
	let fd = match unsafe { ctx.read_at::<c_ulong>(16) } {
		Ok(fd) => fd as u32,
		Err(_) => return 0,
	};
	let vec = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(vec) => vec as *mut iovec,
		Err(_) => return 0,
	};
	let vlen = match unsafe { ctx.read_at::<c_ulong>(32) } {
		Ok(vlen) => vlen as u64,
		Err(_) => return 0,
	};
	let seq = match read_seq(fd) {
		Ok(seq) => seq,
		Err(_) => return 0,
	};
	let args = Args::vectored(fd, seq, vec, vlen, timestamp);
	try_enter(ctx, args, SyscallType::Ingress).unwrap_or_else(|ret| ret)
}
/// name: sys_exit_readv
/// ID: 692
///
///     field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_readv")]
fn sys_exit_readv(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let ret = match unsafe { ctx.read_at::<c_long>(16) } {
		Ok(ret) => ret,
		Err(_) => return 0,
	};
	try_exit(ctx, ret, SyscallName::ReadV, SyscallType::Ingress).unwrap_or_else(|ret| ret)
}
