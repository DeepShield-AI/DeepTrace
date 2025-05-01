use aya_ebpf::helpers::gen::bpf_get_prandom_u32;
use mercury_common::{consts::MAX_INFER_PAYLOAD_SIZE, protocols::L7Protocol, structs::Direction};

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SocketInfo {
	pub uuid: u32,
	pub exit_seq: u32,
	pub direction: Direction,
	pub pre_direction: Direction,
	pub l7protocol: L7Protocol,
	padding: u8,
	pub prev_len: u32,
	pub prev_buf: [u8; MAX_INFER_PAYLOAD_SIZE as usize],
}

impl SocketInfo {
	pub fn new() -> Self {
		Self {
			uuid: unsafe { bpf_get_prandom_u32() },
			exit_seq: 0,
			pre_direction: Direction::Unknown,
			direction: Direction::Unknown,
			l7protocol: L7Protocol::Unknown,
			padding: 0,
			prev_len: 0,
			prev_buf: [0; MAX_INFER_PAYLOAD_SIZE as usize],
		}
	}
}
