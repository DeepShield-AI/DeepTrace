use crate::{
	network::process::{try_enter, try_exit},
	structs::Args,
	utils::{is_filtered_pid, write_seq},
	vmlinux::user_msghdr,
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::{bpf_ktime_get_ns, gen::bpf_probe_read},
	macros::tracepoint,
	programs::TracePointContext,
};
use core::mem::MaybeUninit;
use mercury_common::{SyscallName, SyscallType};

/// name: sys_enter_sendmsg
/// ID: 1417
///
///     field:int fd;   offset:16;      size:8; signed:0;
///     field:struct user_msghdr * msg; offset:24;      size:8; signed:0;
///     field:unsigned int flags;       offset:32;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_sendmsg")]
fn sys_enter_sendmsg(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let timestamp = unsafe { bpf_ktime_get_ns() };
	let fd = match unsafe { ctx.read_at::<c_ulong>(16) } {
		Ok(fd) => fd as u32,
		Err(_) => return 0,
	};
	let msg = match unsafe { ctx.read_at::<c_ulong>(24) } {
		Ok(msg) => {
			let mut v: MaybeUninit<user_msghdr> = MaybeUninit::uninit();
			let ret = unsafe {
				bpf_probe_read(
					v.as_mut_ptr() as *mut _,
					size_of::<user_msghdr>() as u32,
					msg as *const _,
				)
			};
			match ret {
				0 => unsafe { v.assume_init() },
				_ => return 0,
			}
		},
		Err(_) => return 0,
	};
	let seq = match write_seq(fd) {
		Ok(seq) => seq,
		Err(_) => return 0,
	};
	let args = Args::vectored(fd, seq, msg.msg_iov, msg.msg_iovlen, timestamp);
	try_enter(ctx, args, SyscallType::Egress).unwrap_or_else(|ret| ret)
}
/// name: sys_exit_sendmsg
/// ID: 1416
///
///     field:long ret; offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_sendmsg")]
fn sys_exit_sendmsg(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}
	let ret = match unsafe { ctx.read_at::<c_long>(16) } {
		Ok(ret) => ret,
		Err(_) => return 0,
	};
	try_exit(ctx, ret, SyscallName::SendMsg, SyscallType::Egress).unwrap_or_else(|ret| ret)
}
