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
use trace_common::structs::{Direction, Syscall};

/// `name`: sys_enter_read `ID`: 701
///
///     unsigned int fd;  offset:16;      size:8; signed:0;
///     field:char * buf;       offset:24;      size:8; signed:0;
///     field:size_t count;     offset:32;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_read")]
fn sys_enter_read(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let timestamp = unsafe { bpf_ktime_get_ns() };
	let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
	if fd < 3 {
		return 0;
	}

	let Ok(buf) = (unsafe { ctx.read_at::<c_ulong>(24) }) else { return 0 };
	let Ok(count) = (unsafe { ctx.read_at::<c_ulong>(32) }) else { return 0 };
	let Ok(seq) = read_seq(fd) else { return 0 };

	let args = Args::normal(fd, buf, count, timestamp, seq);
	try_enter(args, Direction::Ingress).unwrap_or_else(|ret| ret)
}
/// `name`: sys_exit_read `ID`: 700
///
///         field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_read")]
fn sys_exit_read(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
	try_exit(ctx, ret, Syscall::Read, Direction::Ingress).unwrap_or_else(|ret| ret)
}
