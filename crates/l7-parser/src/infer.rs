use crate::{
	cassandra::Cassandra, dns::DNS, http1::HTTP1, memcached::Memcached, mongodb::MongoDB,
	mysql::MySQL, redis::Redis, rocketmq::RocketMQ, thrift::Thrift, Classification,
};
use aya_ebpf::programs::TracePointContext;
use aya_log_ebpf::info;
use ebpf_common::{
	alloc,
	error::{Result, code::*},
	utils::is_filtered_comm,
};
use observ_trace_common::{
	Buffer, Direction, L7Protocol, Quintuple, SocketInfo, constants::MAX_INFER_SIZE,
	maps::SOCKET_INFO,
};

pub(crate) trait Infer {
	// TODO: remove ctx arg when protocol parse finished
	fn parse(
		ctx: &TracePointContext,
		quintuple: &Quintuple,
		direction: Direction,
		buffer: &Buffer<MAX_INFER_SIZE>,
		key: u64,
		enter_seq: u32,
		exit_seq: u32,
	) -> Result<Classification>;
}

// TODO: remove ctx arg when protocol parse finished
#[inline(always)]
pub fn protocol_infer(
	ctx: &TracePointContext,
	quintuple: &Quintuple,
	direction: Direction,
	// first [`MAX_INFER_SIZE`] bytes of payload
	buffer: &Buffer<MAX_INFER_SIZE>,
	// socket info map key
	key: u64,
	enter_seq: u32,
	exit_seq: u32,
) -> Result<Classification> {
	if is_filtered_comm() {
		return Ok(Classification::default());
	}
	let mut result =
		protocol_infer_impl(ctx, quintuple, direction, buffer, key, enter_seq, exit_seq);
	let map = unsafe { &SOCKET_INFO };
	let sock_info = {
		match map.get_ptr_mut(&key) {
			Some(ptr) => &mut unsafe { *ptr },
			None => {
				let sock_info = alloc::alloc_zero::<SocketInfo>()?;
				map.insert(&key, sock_info, 0).map_err(|_| MAP_INSERT_FAILED)?;
				let ptr = map.get_ptr_mut(&key).ok_or(0_u32)?;
				&mut unsafe { *ptr }
			},
		}
	};

	if result.protocol == L7Protocol::Unknown && buffer.len() <= MAX_INFER_SIZE {
		sock_info.pre_direction = sock_info.direction;
		sock_info.direction = direction;
		sock_info.prev_buf = buffer.clone();
		sock_info.exit_seq = exit_seq;
		map.insert(&key, sock_info, 0).map_err(|_| MAP_INSERT_FAILED)?;
		Err(PREV_PAYLOAD_SAVED)
	} else {
		if sock_info.l7protocol == L7Protocol::Unknown {
			sock_info.l7protocol = result.protocol;
		}
		result.seq = sock_info.seq;
		result.uuid = sock_info.uuid;
		sock_info.seq += 1;
		map.insert(&key, sock_info, 0).map_err(|_| MAP_INSERT_FAILED)?;
		Ok(result)
	}
}

#[inline(always)]
fn protocol_infer_impl(
	ctx: &TracePointContext,
	quintuple: &Quintuple,
	direction: Direction,
	buffer: &Buffer<MAX_INFER_SIZE>,
	key: u64,
	enter_seq: u32,
	exit_seq: u32,
) -> Classification {
	let _skip = L7Protocol::Unknown;
	// TODO: + 用户态可配置的逻辑
	[
		Redis::parse,
		Thrift::parse,
		Memcached::parse,
		MongoDB::parse,
		DNS::parse,
		HTTP1::parse,
		Cassandra::parse,
		MySQL::parse,
		RocketMQ::parse,
	]
	.iter()
	.find_map(|parser| parser(ctx, quintuple, direction, buffer, key, enter_seq, exit_seq).ok())
	.unwrap_or_default()
}
