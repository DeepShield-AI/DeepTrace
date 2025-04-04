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
use mercury_common::{SyscallName, SyscallType};

/// `name`: sys_enter_sendto
/// `ID`: 1427
///
///     field:int fd;   offset:16;      size:8; signed:0;
///     field:void * ubuf;      offset:24;      size:8; signed:0;
///     field:size_t len;       offset:32;      size:8; signed:0;
///     field:unsigned int flags;       offset:40;      size:8; signed:0;
///     field:struct sockaddr * addr;   offset:48;      size:8; signed:0;
///     field:int * addr_len;   offset:56;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_sendto")]
fn sys_enter_sendto(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let timestamp = unsafe { bpf_ktime_get_ns() };
	let fd = match unsafe { ctx.read_at::<c_ulong>(16) } {
		Ok(fd) => fd,
		Err(_) => return 0,
	};
	let ubuf = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(ubuf) => ubuf,
		Err(_) => return 0,
	};
	let len = match unsafe { ctx.read_at::<c_ulong>(32) } {
		Ok(len) => len,
		Err(_) => return 0,
	};
	let seq = match write_seq(fd) {
		Ok(seq) => seq,
		Err(_) => return 0,
	};
	let args = Args::normal(fd, seq, ubuf, len, timestamp);
	try_enter(ctx, args, SyscallType::Egress).unwrap_or_else(|ret| ret)
}
/// `name`: sys_exit_sendto
/// `ID`: 1426
///
///     field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_sendto")]
fn sys_exit_sendto(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let ret = match unsafe { ctx.read_at::<c_long>(16) } {
		Ok(ret) => ret,
		Err(_) => return 0,
	};
	try_exit(ctx, ret, SyscallName::SendTo, SyscallType::Egress).unwrap_or_else(|ret| ret)
}
