use super::{
	dns::DNS, http1::HTTP1, memcached::Memcached, mongodb::MongoDB, mysql::MySQL, redis::Redis,
	thrift::Thrift,
};
use crate::{
	constants::MAX_INFER_PAYLOAD_SIZE,
	maps::{INFER, SOCKET_INFO},
	structs::{Args, InferInfo, SocketInfo},
	utils::{gen_connect_key, is_filtered_comm},
};
use aya_ebpf::{helpers::bpf_get_current_pid_tgid, programs::TracePointContext};
use aya_log_ebpf::info;
use trace_common::{
	message::Message,
	protocols::L7Protocol,
	structs::{Direction, Quintuple},
};

pub trait Infer {
	fn parse(
		ctx: &TracePointContext,
		info: &InferInfo,
		quintuple: Quintuple,
	) -> Result<Message, u32>;
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
		let ptr = INFER.get_ptr_mut(0).ok_or(0_u32)?;
		&mut *ptr
	};
	let len = args.infer_extract(info.buf.as_mut_ptr(), count)?;
	info.len = len;
	let key = gen_connect_key(bpf_get_current_pid_tgid(), args.fd);
	info.key = key;
	info.count = count;
	info.enter_seq = args.enter_seq;
	info.exit_seq = exit_seq;
	info.direction = direction;

	let mut message = infer_protocol_impl(ctx, info, quintuple);
	let map = unsafe { &SOCKET_INFO };
	let sock_info = {
		match map.get_ptr_mut(&key) {
			Some(ptr) => &mut unsafe { *ptr },
			None => {
				let sock_info = SocketInfo::new();
				map.insert(&key, &sock_info, 0).map_err(|e| e as u32)?;
				let ptr = map.get_ptr_mut(&key).ok_or(0_u32)?;
				&mut unsafe { *ptr }
			},
		}
	};
	if message.protocol == L7Protocol::Unknown {
		info!(ctx, "infer protocol error");
		info!(
			ctx,
			"{} {} {} {} {} {} {} {}",
			info.buf[0],
			info.buf[1],
			info.buf[2],
			info.buf[3],
			info.buf[4],
			info.buf[5],
			info.buf[6],
			info.buf[7]
		);
		// info!(ctx, "{} {} {} {}", info.buf[0], info.buf[1], info.buf[2], info.buf[3]);
		info!(ctx, "{} {}", info.count, info.len);
	}
	if message.protocol == L7Protocol::Unknown && info.count <= MAX_INFER_PAYLOAD_SIZE {
		sock_info.pre_direction = sock_info.direction;
		sock_info.direction = info.direction;
		let copy_size = args.save_prev(sock_info.prev_buf.as_mut_ptr())?;
		sock_info.prev_len = copy_size;
		sock_info.exit_seq = info.exit_seq;
		map.insert(&key, sock_info, 0).map_err(|e| e as u32)?;
		Err(0)
	} else {
		if sock_info.l7protocol == L7Protocol::Unknown {
			sock_info.l7protocol = message.protocol;
			map.insert(&key, sock_info, 0).map_err(|e| e as u32)?;
		}
		message.uuid = sock_info.uuid;
		Ok(message)
	}
}

fn infer_protocol_impl(ctx: &TracePointContext, info: &InferInfo, quintuple: Quintuple) -> Message {
	let _skip = L7Protocol::Unknown;
	// TODO: + 用户态可配置的逻辑
	[
		Redis::parse,
		Thrift::parse,
		Memcached::parse,
		MongoDB::parse,
		DNS::parse,
		HTTP1::parse,
		MySQL::parse,
	]
	.iter()
	.find_map(|parser| parser(ctx, info, quintuple).ok())
	.unwrap_or_default()
}
