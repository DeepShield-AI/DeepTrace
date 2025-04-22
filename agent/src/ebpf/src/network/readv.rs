use crate::{
	network::process::{try_enter, try_exit},
	structs::Args,
	utils::{is_filtered_pid, read_seq},
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::bpf_ktime_get_ns,
	macros::tracepoint,
	programs::TracePointContext,
};
use mercury_common::structs::{Direction, Syscall};

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

	let timestamp = unsafe { bpf_ktime_get_ns() };
	let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
	let Ok(vec) = (unsafe { ctx.read_at::<c_ulong>(24) }) else { return 0 };
	let Ok(vlen) = (unsafe { ctx.read_at::<c_ulong>(32) }) else { return 0 };
	let Ok(seq) = read_seq(fd) else { return 0 };

	let args = Args::vectored(fd, vec, vlen, timestamp, seq);
	try_enter(args, Direction::Ingress).unwrap_or_else(|ret| ret)
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

	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };

	try_exit(ctx, ret, Syscall::ReadV, Direction::Ingress).unwrap_or_else(|ret| ret)
}
