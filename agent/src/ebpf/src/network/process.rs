#![allow(static_mut_refs)]

use crate::{
	maps::{DATA, EGRESS, EVENTS, INGRESS},
	structs::Args,
	utils::{quintuple_from_sock, tcp_sock_from_fd},
};
use aya_ebpf::{
	cty::c_long,
	helpers::{bpf_get_current_pid_tgid, bpf_map_update_elem, gen::{bpf_get_current_comm, bpf_ktime_get_ns}},
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
	let ret = ret as u64;
	let args = unsafe { map.get(&id).ok_or(0_u32)? };
	let data = unsafe {
		let data_ptr = DATA.get_ptr_mut(0).ok_or(0_u32)?;
		&mut *data_ptr
	};
	let sock = tcp_sock_from_fd(args.fd())?;
	data.tgid = ctx.tgid();
	data.pid = ctx.pid();
	data.comm = ctx.command().map_err(|e| e as u32)?;
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
	let copy_size = args.extract(data.buf.as_mut_ptr(), ret)?;
	data.len = copy_size;
	// collect metrics
	data.srtt_us = unsafe { &*sock }.srtt_us;
	data.mdev_max_us = unsafe { &*sock }.mdev_max_us;
	data.rttvar_us = unsafe { &*sock }.rttvar_us;
	data.mdev_us = unsafe { &*sock }.mdev_us;
	data.bytes_sent = unsafe { &*sock }.bytes_sent;
	data.bytes_received = unsafe { &*sock }.bytes_received;
	data.bytes_acked = unsafe { &*sock }.bytes_acked;
	data.delivered = unsafe { &*sock }.delivered;
	data.snd_cwnd = unsafe { &*sock }.snd_cwnd;
	data.rtt_us = unsafe { &*sock }.rcv_rtt_est.rtt_us;

	data.syscall = syscall;
	data.direction = direction;

	map.remove(&id).map_err(|e| e as u32)?;

	unsafe { EVENTS.output(&ctx, &(*data), 0) };

	Ok(0)
}
