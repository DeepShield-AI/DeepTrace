use cache::{Cache, CacheEntry, SessionKey};
use construct::construct_spans;
pub use error::Error as SpanError;
pub use module::SpanConstructor;
use serde::Serialize;
use std::ffi::CStr;
use trace_common::{
	protocols::L7Protocol,
	structs::{Data, Direction, Payload, Syscall},
};

mod cache;
mod construct;
mod error;
mod module;

#[derive(Serialize)]
pub struct Span {
	tgid: u32,
	pid: u32,
	component: String,
	direction: Direction,
	start_time: u64,
	end_time: u64,

	src_ip: u32,
	dst_ip: u32,
	src_port: u16,
	dst_port: u16,

	req_syscall: Syscall,
	resp_syscall: Syscall,
	#[serde(serialize_with = "serialize_payload")]
	req_content: Payload,
	#[serde(serialize_with = "serialize_payload")]
	resp_content: Payload,
	protocol: L7Protocol,
}

impl Span {
	pub fn new(req: Data, resp: Data) -> Self {
		let (src_ip, dst_ip, src_port, dst_port) = match req.direction {
			Direction::Egress => (
				req.quintuple.dst_addr,
				req.quintuple.src_addr,
				req.quintuple.dst_port,
				req.quintuple.src_port,
			),
			_ => (
				req.quintuple.src_addr,
				req.quintuple.dst_addr,
				req.quintuple.src_port,
				req.quintuple.dst_port,
			),
		};
		Self {
			tgid: req.tgid,
			pid: req.pid,
			component: CStr::from_bytes_until_nul(&req.comm)
				.unwrap()
				.to_string_lossy()
				.into_owned(),
			direction: req.direction,
			start_time: req.timestamp_ns,
			end_time: resp.timestamp_ns,
			src_ip,
			dst_ip,
			src_port,
			dst_port,
			req_syscall: req.syscall,
			resp_syscall: resp.syscall,
			req_content: req.payload,
			resp_content: resp.payload,
			protocol: req.protocol,
		}
	}
}

fn serialize_payload<S>(payload: &Payload, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	let s = String::from_utf8_lossy(&payload.buf[..payload.len as usize]);
	serializer.serialize_str(&s)
}
