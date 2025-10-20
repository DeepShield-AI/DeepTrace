#![allow(static_mut_refs)]
use crate::{
	maps::{EGRESS, EVENTS, INGRESS},
	types::Args,
	utils::{gen_connect_key, is_tcp_udp, quintuple_from_sock, tcp_sock_from_fd},
};
use aya_ebpf::{
	EbpfContext,
	cty::c_long,
	helpers::{bpf_get_current_pid_tgid, r#gen::bpf_ktime_get_ns},
	programs::TracePointContext,
};
use aya_log_ebpf::info;
use ebpf_common::{
	alloc,
	buffer::Buffer,
	constants::MAX_PAYLOAD_SIZE,
	error::{Result, code::*},
};
use l7_parser::protocol_infer;
use observ_trace_common::{
	Direction, Message, SocketInfo, Syscall, constants::MAX_INFER_SIZE, maps::SOCKET_INFO,
};

/// Processing enter of `read`, `readv`, `recvfrom`, `recvmsg`, `recvmmsg`, `write`, `writev`, `sendto`, `sendmsg`, `sendmmsg` syscalls
#[inline(always)]
pub fn try_enter(args: Args, direction: Direction) -> Result<u32> {
	let id = bpf_get_current_pid_tgid();

	let map = match direction {
		Direction::Ingress => unsafe { &INGRESS },
		Direction::Egress => unsafe { &EGRESS },
		Direction::Unknown => return Err(INVALID_DIRECTION),
	};

	map.insert(&id, &args, 0).map_err(|_| MAP_INSERT_FAILED)?;
	Ok(0)
}

#[inline(always)]
pub fn try_exit(
	ctx: &TracePointContext,
	ret: c_long,
	syscall: Syscall,
	direction: Direction,
) -> Result<u32> {
	let id = bpf_get_current_pid_tgid();
	let map = match direction {
		Direction::Ingress => unsafe { &INGRESS },
		Direction::Egress => unsafe { &EGRESS },
		Direction::Unknown => return Err(INVALID_DIRECTION),
	};

	if !(0 < ret && ret <= MAX_PAYLOAD_SIZE as i64) {
		map.remove(&id).map_err(|_| MAP_DELETE_FAILED)?;
		info!(ctx, "invalid ret: {}", ret);
		return Err(SYSCALL_PAYLOAD_LENGTH_INVALID);
	}

	let ret = ret as u32;
	let args = match unsafe { map.get(&id) } {
		Some(a) => a,
		None => return Err(MAP_GET_FAILED),
	};

	alloc::init()?;
	let data = alloc::alloc_zero::<Message>()?;
	let sock = tcp_sock_from_fd(args.fd)?;
	let key = gen_connect_key(bpf_get_current_pid_tgid(), args.fd);

	let quintuple = quintuple_from_sock(sock)?;
	data.quintuple = quintuple;
	data.quintuple.l4_protocol = is_tcp_udp(sock)?;

	data.tgid = ctx.tgid();
	data.pid = ctx.pid();
	data.comm = Buffer::from_slice(&ctx.command().map_err(|_| FAILED_TO_GET_COMM)?);
	data.enter_seq = args.enter_seq;

	data.exit_seq = match direction {
		Direction::Ingress => sock.copied_seq().ok_or(READ_TCP_SOCK_COPIED_SEQ_FAILED)?,
		Direction::Egress => sock.write_seq().ok_or(READ_TCP_SOCK_WRITE_SEQ_FAILED)?,
		_ => return Err(INVALID_DIRECTION),
	};

	let infer_payload = alloc::alloc_zero::<Buffer<MAX_INFER_SIZE>>()?;
	args.extract(infer_payload, ret)?;

	let result = protocol_infer(
		ctx,
		&quintuple,
		direction,
		infer_payload,
		key,
		args.enter_seq,
		data.exit_seq,
	)?;
	data.timestamp_ns = unsafe { bpf_ktime_get_ns() };
	data.syscall = syscall;
	data.direction = direction;

	data.type_ = result.type_;
	data.protocol = result.protocol;
	data.seq = result.seq;
	data.uuid = result.uuid;
	// args.extract(&mut data.payload, ret)?;

	map.remove(&id).map_err(|_| MAP_DELETE_FAILED)?;

	unsafe { EVENTS.output(ctx, data.encode(), 0) };

	Ok(0)
}

#[inline(always)]
pub fn try_close(fd: u64) -> Result<u32> {
	let key = gen_connect_key(bpf_get_current_pid_tgid(), fd);
	let map = unsafe { &SOCKET_INFO };
	if unsafe { map.get(&key) }.is_some() {
		map.remove(&key).map_err(|_| MAP_DELETE_FAILED)?;
	}
	Ok(0)
}

#[inline(always)]
pub fn try_socket(fd: u64) -> Result<u32> {
	let key = gen_connect_key(bpf_get_current_pid_tgid(), fd);
	let map = unsafe { &SOCKET_INFO };
	alloc::init()?;
	let socket_info = alloc::alloc_zero::<SocketInfo>()?;
	map.insert(&key, socket_info, 0).map_err(|_| MAP_INSERT_FAILED)?;
	Ok(0)
}
