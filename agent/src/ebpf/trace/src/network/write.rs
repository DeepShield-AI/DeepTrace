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

/// `name`: sys_enter_write
/// `ID`: 699
///
///     field:unsigned int fd;  offset:16;      size:8; signed:0;
///     field:const char * buf; offset:24;      size:8; signed:0;
///     field:size_t count;     offset:32;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_write")]
fn sys_enter_write(ctx: TracePointContext) -> u32 {
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
	let Ok(seq) = write_seq(fd) else { return 0 };

	let args = Args::normal(fd, buf, count, timestamp, seq);
	try_enter(args, Direction::Egress).unwrap_or_else(|ret| ret)
}
/// `name`: sys_exit_write
/// `ID`: 698
///
///         field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_write")]
fn sys_exit_write(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
	try_exit(ctx, ret, Syscall::Write, Direction::Egress).unwrap_or_else(|ret| ret)
}
