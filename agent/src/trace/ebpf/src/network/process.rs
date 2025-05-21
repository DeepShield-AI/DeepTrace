#![allow(static_mut_refs)]
use crate::{
	maps::{DATA, EGRESS, EVENTS, INGRESS, SOCKET_INFO},
	protocols::infer_protocol,
	structs::{Args, SocketInfo},
	utils::{gen_connect_key, is_tcp_udp, quintuple_from_sock, tcp_sock_from_fd},
	vmlinux::tcp_sock,
};
use aya_ebpf::{
	cty::c_long,
	helpers::{bpf_get_current_pid_tgid, gen::bpf_ktime_get_ns},
	programs::TracePointContext,
	EbpfContext,
};
use trace_common::{
	constants::MAX_PAYLOAD_SIZE,
	structs::{Direction, Syscall},
};

/// Processing enter of `read`, `readv`, `recvfrom`, `recvmsg`, `recvmmsg` syscalls
#[inline]
pub fn try_enter(args: Args, direction: Direction) -> Result<u32, u32> {
	let id = bpf_get_current_pid_tgid();

	let map = match direction {
		Direction::Ingress => unsafe { &INGRESS },
		Direction::Egress => unsafe { &EGRESS },
		Direction::Unknown => return Err(0_u32),
	};

	if unsafe { map.get(&id) }.is_some() {
		map.remove(&id).map_err(|e| e as u32)?;
	}
	map.insert(&id, &args, 0).map_err(|e| e as u32)?;
	Ok(0)
}

pub fn try_exit(
	ctx: TracePointContext,
	ret: c_long,
	syscall: Syscall,
	direction: Direction,
) -> Result<u32, u32> {
	let id = bpf_get_current_pid_tgid();
	let map = match direction {
		Direction::Ingress => unsafe { &INGRESS },
		Direction::Egress => unsafe { &EGRESS },
		Direction::Unknown => return Err(0_u32),
	};

	if !(0 < ret && ret <= MAX_PAYLOAD_SIZE as i64) {
		map.remove(&id).map_err(|e| e as u32)?;
		return Err(0);
	}

	let ret = ret as u64;
	let args = {
		let ptr = map.get_ptr_mut(&id).ok_or(0_u32)?;
		&mut unsafe { *ptr }
	};
	let data = unsafe {
		let ptr = DATA.get_ptr_mut(0).ok_or(0_u32)?;
		&mut *ptr
	};
	let sock = tcp_sock_from_fd(args.fd)? as *const tcp_sock;
	let mut quintuple = quintuple_from_sock(sock);

	data.tgid = ctx.tgid();
	data.pid = ctx.pid();
	data.comm = ctx.command().map_err(|e| e as u32)?;

	quintuple.l4_protocol = is_tcp_udp(sock)?;
	data.quintuple = quintuple;
	data.enter_seq = args.enter_seq;
	let exit_seq = match direction {
		Direction::Ingress => unsafe { &*sock }.copied_seq,
		Direction::Egress => unsafe { &*sock }.write_seq,
		_ => return Err(0_u32),
	};
	data.exit_seq = exit_seq;
	data.timestamp_ns = unsafe { bpf_ktime_get_ns() };

	let msg = infer_protocol(&ctx, args, quintuple, direction, exit_seq, ret as u32)?;
	data.protocol = msg.protocol;
	data.type_ = msg.type_;
	data.uuid = msg.uuid;

	let copy_size = args.extract(data.payload.buf.as_mut_ptr(), ret as u32)?;
	data.payload.len = copy_size;

	data.syscall = syscall;
	data.direction = direction;

	map.remove(&id).map_err(|e| e as u32)?;

	unsafe { EVENTS.output(&ctx, &(*data), 0) };

	Ok(0)
}
#[inline]
pub fn try_close(_ctx: TracePointContext, fd: u64) -> Result<u32, u32> {
	let key = gen_connect_key(bpf_get_current_pid_tgid(), fd);
	let map = unsafe { &SOCKET_INFO };
	if unsafe { map.get(&key) }.is_some() {
		map.remove(&key).map_err(|e| e as u32)?;
	}
	Ok(0)
}
#[inline]
pub fn try_socket(fd: u64) -> Result<u32, u32> {
	let key = gen_connect_key(bpf_get_current_pid_tgid(), fd);
	let map = unsafe { &SOCKET_INFO };
	let sock = SocketInfo::new();
	map.insert(&key, &sock, 0).map_err(|e| e as u32)?;
	Ok(0)
}
