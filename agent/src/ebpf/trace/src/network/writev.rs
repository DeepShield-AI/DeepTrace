use crate::{
	network::process::{try_enter, try_exit},
	structs::Args,
	utils::{is_filtered_pid, write_seq},
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::bpf_ktime_get_ns,
	macros::tracepoint,
	programs::TracePointContext,
};
use trace_common::structs::{Direction, Syscall};

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
	let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
	let Ok(vec) = (unsafe { ctx.read_at::<c_ulong>(24) }) else { return 0 };
	let Ok(vlen) = (unsafe { ctx.read_at::<c_ulong>(32) }) else { return 0 };
	let Ok(seq) = write_seq(fd) else { return 0 };

	let args = Args::vectored(fd, vec, vlen, timestamp, seq);
	try_enter(args, Direction::Egress).unwrap_or_else(|ret| ret)
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

	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
	try_exit(ctx, ret, Syscall::WriteV, Direction::Egress).unwrap_or_else(|ret| ret)
}
