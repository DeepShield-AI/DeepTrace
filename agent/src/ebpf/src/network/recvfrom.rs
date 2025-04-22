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

/// `name`: sys_enter_recvfrom
/// `ID`: 1425
///
///     field:int fd;   offset:16;      size:8; signed:0;
///     field:void * ubuf;      offset:24;      size:8; signed:0;
///     field:size_t size;      offset:32;      size:8; signed:0;
///     field:unsigned int flags;       offset:40;      size:8; signed:0;
///     field:struct sockaddr * addr;   offset:48;      size:8; signed:0;
///     field:int * addr_len;   offset:56;      size:8; signed:0;
// TODO: flags, addr and addr_len handle
#[tracepoint(category = "syscalls", name = "sys_enter_recvfrom")]
fn sys_enter_recvfrom(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let timestamp = unsafe { bpf_ktime_get_ns() };
	let Ok(flags) = (unsafe { ctx.read_at::<c_ulong>(40) }) else { return 0 };
	// If flags contains MSG_PEEK, it is returned directly.
	// ref : https://linux.die.net/man/2/recvfrom
	if flags & 0x02 != 0 {
		return 0;
	}

	let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
	let Ok(ubuf) = (unsafe { ctx.read_at::<c_ulong>(24) }) else { return 0 };
	let Ok(size) = (unsafe { ctx.read_at::<c_ulong>(32) }) else { return 0 };
	let Ok(seq) = read_seq(fd) else { return 0 };

	let args = Args::normal(fd, ubuf, size, timestamp, seq);
	try_enter(args, Direction::Ingress).unwrap_or_else(|ret| ret)
}
/// `name`: sys_exit_recvfrom
/// `ID`: 1424
///
///     field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_recvfrom")]
fn sys_exit_recvfrom(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
	try_exit(ctx, ret, Syscall::RecvFrom, Direction::Ingress).unwrap_or_else(|ret| ret)
}
