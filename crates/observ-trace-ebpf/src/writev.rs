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
use ebpf_common::{co_re::iovec, try_or_log};
use observ_trace_common::{Direction, Syscall};

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
	let vec = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(vec) if vec != 0 => iovec::from_ptr(vec as *const _),
		_ => return 0,
	};
	let vlen = match unsafe { ctx.read_at::<c_ulong>(32) } {
		Ok(vlen) if vlen != 0 => vlen as u32,
		_ => return 0,
	};
	let Ok(seq) = write_seq(fd) else { return 0 };

	let args = Args::from_msg(fd, vec, vlen, timestamp, seq);
	try_or_log!(&ctx, try_enter(args, Direction::Egress))
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
	try_or_log!(&ctx, try_exit(&ctx, ret, Syscall::WriteV, Direction::Egress))
}
