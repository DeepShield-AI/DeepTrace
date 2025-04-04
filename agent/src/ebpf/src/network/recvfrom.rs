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
use mercury_common::{SyscallName, SyscallType};

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
	let flags = match unsafe { ctx.read_at::<c_ulong>(40) } {
		Ok(flags) => flags,
		Err(_) => return 0,
	};
	// If flags contains MSG_PEEK, it is returned directly.
	// ref : https://linux.die.net/man/2/recvfrom
	if flags & 0x02 != 0 {
		return 0;
	}
	let fd = match unsafe { ctx.read_at::<c_ulong>(16) } {
		Ok(fd) => fd,
		Err(_) => return 0,
	};
	let ubuf = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(ubuf) => ubuf,
		Err(_) => return 0,
	};
	let size = match unsafe { ctx.read_at::<c_ulong>(32) } {
		Ok(size) => size,
		Err(_) => return 0,
	};
	let seq = match read_seq(fd) {
		Ok(seq) => seq,
		Err(_) => return 0,
	};
	let args = Args::normal(fd, seq, ubuf, size, timestamp);
	try_enter(ctx, args, SyscallType::Ingress).unwrap_or_else(|ret| ret)
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
	let ret = match unsafe { ctx.read_at::<c_long>(16) } {
		Ok(ret) => ret,
		Err(_) => return 0,
	};
	try_exit(ctx, ret, SyscallName::RecvFrom, SyscallType::Ingress).unwrap_or_else(|ret| ret)
}
