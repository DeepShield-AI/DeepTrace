use crate::{
	process::{try_enter, try_exit},
	types::Args,
	utils::{is_filtered_pid, read_seq},
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::bpf_ktime_get_ns,
	macros::tracepoint,
	programs::TracePointContext,
};
use ebpf_common::{co_re::user_msghdr, try_or_log};
use observ_trace_common::{Direction, Syscall};

/// name: sys_enter_recvmsg
/// ID: 1413
///
///     field:int fd;   offset:16;      size:8; signed:0;
///     field:struct user_msghdr * msg; offset:24;      size:8; signed:0;
///     field:unsigned int flags;       offset:32;      size:8; signed:0;
// TODO: flags handle
#[tracepoint(category = "syscalls", name = "sys_enter_recvmsg")]
fn sys_enter_recvmsg(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let timestamp = unsafe { bpf_ktime_get_ns() };
	let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
	let (vec, vlen) = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(msg) if msg != 0 => {
			let msg = user_msghdr::from_ptr(msg as *const _);
			match (msg.msg_iov(), msg.msg_iovlen()) {
				(Some(vec), Some(vlen)) if !vec.is_null() && vlen != 0 => (vec, vlen as u32),
				_ => return 0,
			}
		},
		_ => return 0,
	};
	let Ok(seq) = read_seq(fd) else { return 0 };

	let args = Args::from_msg(fd, vec, vlen, timestamp, seq);
	try_or_log!(&ctx, try_enter(args, Direction::Ingress))
}
/// name: sys_exit_recvmsg
/// ID: 1412
///
///     field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_recvmsg")]
fn sys_exit_recvmsg(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
	try_or_log!(&ctx, try_exit(&ctx, ret, Syscall::RecvMsg, Direction::Ingress))
}
