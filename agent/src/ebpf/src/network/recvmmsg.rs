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

/// Send multiple messages to a socket
/// #include <sys/types.h>
/// ```c
/// #include <sys/socket.h>
/// int sendmmsg(int s, struct mmsghdr *mmsg, unsigned int vlen, unsigned int flags)
/// ```
/// name: sys_enter_recvmmsg
/// ID: 1411
///
///     field:int fd;   offset:16;      size:8; signed:0;
///     field:struct mmsghdr * mmsg;    offset:24;      size:8; signed:0;
///     field:unsigned int vlen;        offset:32;      size:8; signed:0;
///     field:unsigned int flags;       offset:40;      size:8; signed:0;
///     field:struct __kernel_timespec * timeout;       offset:48;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_recvmmsg")]
fn sys_enter_recvmmsg(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let timestamp = unsafe { bpf_ktime_get_ns() };
	let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
	let Ok(mmsg) = (unsafe { ctx.read_at::<c_ulong>(24) }) else { return 0 };
	let Ok(vlen) = (unsafe { ctx.read_at::<c_ulong>(32) }) else { return 0 };
	let Ok(seq) = read_seq(fd) else { return 0 };

	let args = Args::msg(fd, mmsg, vlen, timestamp, seq);
	try_enter(args, Direction::Ingress).unwrap_or_else(|ret| ret)
}
/// name: sys_exit_recvmmsg
/// ID: 1410
///
///     field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_recvmmsg")]
fn sys_exit_recvmmsg(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
	try_exit(ctx, ret, Syscall::RecvMMsg, Direction::Ingress).unwrap_or_else(|ret| ret)
}
