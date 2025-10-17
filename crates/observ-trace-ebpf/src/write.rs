use crate::{
	process::{try_enter, try_exit},
	types::Args,
	utils::{is_filtered_pid, write_seq},
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::bpf_ktime_get_ns,
	macros::tracepoint,
	programs::TracePointContext,
};
use ebpf_common::try_or_log;
use observ_trace_common::structs::{Direction, Syscall};

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
	let buf = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(buf) if buf != 0 => buf as *mut u8,
		_ => return 0,
	};
	let count = match unsafe { ctx.read_at::<c_ulong>(32) } {
		Ok(count) if count != 0 => count as u32,
		_ => return 0,
	};
	let Ok(seq) = write_seq(fd) else { return 0 };

	let args = Args::from_ubuf(fd, buf, count, timestamp, seq);
	try_or_log!(&ctx, try_enter(args, Direction::Egress))
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
	try_or_log!(&ctx, try_exit(&ctx, ret, Syscall::Write, Direction::Egress))
}
