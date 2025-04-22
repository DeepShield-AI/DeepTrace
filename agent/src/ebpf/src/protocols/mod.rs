#![allow(static_mut_refs)]
use crate::{
	maps::{INFER, SOCKET_INFO},
	structs::{Args, InferInfo, SocketInfo},
	utils::{gen_connect_key, is_filtered_comm},
};
use aya_ebpf::{helpers::bpf_get_current_pid_tgid, programs::TracePointContext};
use memcached::Memcached;
use mercury_common::{
	consts::MAX_INFER_PAYLOAD_SIZE,
	message::Message,
	protocols::L7Protocol,
	structs::{Direction, Quintuple},
};
use mongodb::MongoDB;
use redis::Redis;
use thrift::Thrift;

mod dns;
mod memcached;
mod mongodb;
mod redis;
mod thrift;
mod utils;
pub trait Infer {
	fn parse(ctx: &TracePointContext, info: &InferInfo, quintuple: Quintuple) -> Result<Message, u32>;
}
// TODO: remove ctx arg when protocol parse finished
pub(crate) fn infer_protocol(
	ctx: &TracePointContext,
	args: &Args,
	quintuple: Quintuple,
	direction: Direction,
	exit_seq: u32,
	count: u32,
) -> Result<Message, u32> {
	if is_filtered_comm() {
		return Ok(Message::default());
	}

	let info = unsafe {
		let infer_ptr = INFER.get_ptr_mut(0).ok_or(0_u32)?;
		&mut *infer_ptr
	};

	let len = args.infer_extract(info.buf.as_mut_ptr(), MAX_INFER_PAYLOAD_SIZE)?;
	info.len = len;

	let key = gen_connect_key(bpf_get_current_pid_tgid(), args.fd);
	info.key = key;
	info.count = count;
	info.enter_seq = args.enter_seq;
	info.exit_seq = exit_seq;
	info.direction = direction;

	let message = infer_protocol_impl(ctx, info, quintuple);

	let map = unsafe { &SOCKET_INFO };
	let mut sock_info = SocketInfo::new();
	if message.protocol == L7Protocol::Unknown {
		if info.count <= MAX_INFER_PAYLOAD_SIZE {
			if let Some(socket) = unsafe { map.get(&key) } {
				sock_info.pre_direction = socket.direction;
			};
			sock_info.direction = info.direction;
			let copy_size = args.save_prev(sock_info.prev_buf.as_mut_ptr())?;
			sock_info.prev_len = copy_size;
			sock_info.exit_seq = info.exit_seq;
			map.insert(&key, &sock_info, 0).map_err(|e| e as u32)?;
			Err(0)
		} else {
			Ok(message)
		}
	} else {
		sock_info.l7protocol = message.protocol;
		map.insert(&key, &sock_info, 0).map_err(|e| e as u32)?;
		Ok(message)
	}
}

fn infer_protocol_impl(ctx: &TracePointContext, info: &InferInfo, quintuple: Quintuple) -> Message {
	let _skip = L7Protocol::Unknown;
	// TODO: + 用户态可配置的逻辑
	[Redis::parse, Thrift::parse, Memcached::parse, MongoDB::parse]
		.iter()
		.find_map(|parser| parser(ctx, info, quintuple).ok())
		.unwrap_or_default()
}
