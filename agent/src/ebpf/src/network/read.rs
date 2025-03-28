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
	let fd = match unsafe { ctx.read_at::<c_ulong>(16) } {
		Ok(fd) => fd as u32,
		Err(_) => return 0,
	};
	let buf: *const u8 = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(buf) => buf as *const u8,
		Err(_) => return 0,
	};
	let count = match unsafe { ctx.read_at::<c_ulong>(32) } {
		Ok(count) => count as u32,
		Err(_) => return 0,
	};
	let seq = match read_seq(fd) {
		Ok(seq) => seq,
		Err(_) => return 0,
	};
	// let tcp_sock = get_tcp_sock_from_fd(&ctx, fd as u32)?;
	// let quintuple =
	let args = Args::normal(fd, seq, buf, count, timestamp);
	try_enter(ctx, args, SyscallType::Ingress).unwrap_or_else(|ret| ret)
}
/// `name`: sys_exit_read `ID`: 700
///
///         field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_read")]
fn sys_exit_read(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let ret = match unsafe { ctx.read_at::<c_long>(16) } {
		Ok(ret) => ret,
		Err(_) => return 0,
	};
	try_exit(ctx, ret, SyscallName::Read, SyscallType::Ingress).unwrap_or_else(|ret| ret)
}
