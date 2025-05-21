use crate::{
	network::process::{try_enter, try_exit},
	structs::Args,
	utils::{is_filtered_pid, read_seq},
	vmlinux::user_msghdr,
};
use aya_ebpf::{
	cty::{c_long, c_ulong},
	helpers::{bpf_ktime_get_ns, r#gen::bpf_probe_read},
	macros::tracepoint,
	programs::TracePointContext,
};
use core::mem::MaybeUninit;
use trace_common::structs::{Direction, Syscall};

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
	let Ok(seq) = read_seq(fd) else { return 0 };

	let args = Args::vectored(fd, msg.msg_iov.addr() as u64, msg.msg_iovlen, timestamp, seq);
	try_enter(args, Direction::Ingress).unwrap_or_else(|ret| ret)
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
	try_exit(ctx, ret, Syscall::RecvMsg, Direction::Ingress).unwrap_or_else(|ret| ret)
}
