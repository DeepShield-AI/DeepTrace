#![allow(static_mut_refs)]

use crate::{
	maps::{EGRESS, INGRESS, MESSAGE},
	structs::Args,
	utils::{quintuple_from_sock, tcp_sock_from_fd},
};
use aya_ebpf::{
	cty::c_long,
	helpers::{bpf_get_current_pid_tgid, gen::bpf_get_current_comm, r#gen::bpf_ktime_get_ns},
	programs::TracePointContext,
	EbpfContext, TASK_COMM_LEN,
};
use mercury_common::{Data, SyscallName, SyscallType, MAX_PAYLOAD_SIZE};

/// Processing enter of `read`, `readv`, `recvfrom`, `recvmsg`, `recvmmsg` syscalls
pub fn try_enter(_ctx: TracePointContext, args: Args, direction: SyscallType) -> Result<u32, u32> {
	let id = bpf_get_current_pid_tgid();
	let map = match direction {
		SyscallType::Ingress => unsafe { &INGRESS },
		SyscallType::Egress => unsafe { &EGRESS },
	};
	if unsafe { map.get(&id) }.is_some() {
		map.remove(&id).map_err(|e| e as u32)?;
	}
	// TODO: add five tuple logic which may be implemented at entries point
	// let tcp_sock = get_tcp_sock_from_fd(&ctx, fd as u32)?;
	// let tcp = unsafe { TCP_SOCK.get_ptr_mut(0).ok_or(0_u32)? };

	map.insert(&id, &args, 0).map_err(|e| e as u32)?;
	Ok(0)
}

pub fn try_exit(
	ctx: TracePointContext,
	ret: c_long,
	syscall: SyscallName,
	direction: SyscallType,
) -> Result<u32, u32> {
	let id = bpf_get_current_pid_tgid();
	let map = match direction {
		SyscallType::Ingress => unsafe { &INGRESS },
		SyscallType::Egress => unsafe { &EGRESS },
	};

	if !(0 < ret && ret <= MAX_PAYLOAD_SIZE as i64) {
		map.remove(&id).map_err(|e| e as u32)?;
		return Err(0);
	}
	let ret = ret as u32;
	let args = unsafe { map.get(&id).ok_or(0_u32)? };

	if let Some(mut message) = unsafe { &MESSAGE }.reserve::<Data>(0) {
		let data = unsafe { &mut *message.as_mut_ptr() };
		data.tgid = ctx.tgid();
		data.pid = ctx.pid();
		unsafe { bpf_get_current_comm(data.comm.as_mut_ptr() as *mut _, TASK_COMM_LEN as u32) };
		let sock = match tcp_sock_from_fd(args.fd()) {
			Ok(sock) => sock,
			Err(_) => {
				message.discard(0);
				map.remove(&id).map_err(|e| e as u32)?;
				return Err(0)
			},
		};
		let quintuple = quintuple_from_sock(sock);
		data.quintuple = quintuple;
		let enter_seq = args.seq();
		data.enter_seq = enter_seq;
		let exit_seq = match direction {
			SyscallType::Ingress => unsafe { &*sock }.copied_seq,
			SyscallType::Egress => unsafe { &*sock }.write_seq,
		};
		data.exit_seq = exit_seq;
		data.timestamp_ns = unsafe { bpf_ktime_get_ns() };
		data.syscall = syscall;
		data.direction = direction;
		let copy_size = match args.extract(data.buf.as_mut_ptr() as *mut _, ret) {
			Ok(size) => size,
			Err(_) => {
				message.discard(0);
				map.remove(&id).map_err(|e| e as u32)?;
				return Err(0)
			},
		};
		data.len = copy_size;
		message.submit(0);
	}
	map.remove(&id).map_err(|e| e as u32)?;

	Ok(0)
}
