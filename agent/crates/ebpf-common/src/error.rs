//! Error type for eBPF programs.
//! ```markdown
//! 32 bits error code
//!
//! 32              16                0
//! |----------------|----------------|
//! |     module     |     error      |
//! ```
pub type Error = u32;
pub type Result<T> = core::result::Result<T, Error>;

macro_rules! define_modules {
	(
		$(
			$name:ident = $value:expr => $description:expr,
		)*
	) => {
		pub mod module {
			$(
				#[doc = concat!($description, " Error")]
				pub const $name: u16 = $value;
			)*
		}

		#[inline(always)]
		const fn module_name(code: u16) -> &'static str {
			match code {
				$(
					module::$name => $description,
				)*
				_ => "Unknown",
			}
		}

		#[inline(always)]
		pub const fn extract_module(code: Error) -> u16 {
			((code >> 16) & 0xFFFF) as u16
		}

		#[inline(always)]
		pub const fn extract_error(code: Error) -> u16 {
			(code & 0xFFFF) as u16
		}
	};
}

define_modules! {
	COMMON = 0x01 => "Common",
	ALLOC = 0x02 => "Alloc",
	BUFFER = 0x03 => "Buffer",
	MAP = 0x04 => "Map",
	CORE = 0x05 => "CO-RE",
	PTR = 0x06 => "Pointer",
	NETWORK = 0x07 => "Network",
	PARSE = 0x08 => "Parse",
}

pub mod code {
	use super::{Error, module};
	macro_rules! error_code {
		($module:expr, $error:expr) => {
			(($module as u32) << 16) | ($error as u32)
		};
	}

	macro_rules! define_error {
		(
			module = $module:expr,
			errors = [
				$($name:ident = $error:expr),* $(,)?
			]
		) => {
			$(
				pub const $name: Error = error_code!($module, $error);
			)*
		};
	}

	define_error! {
		module = module::COMMON,
		errors = [
			SHOULD_NOT_HAPPEN = 0x00,
			INVALID_DIRECTION = 0x01,
			FAILED_TO_GET_COMM = 0x02,
			SYSCALL_PAYLOAD_LENGTH_INVALID = 0x03,
		]
	}

	define_error! {
		module = module::ALLOC,
		errors = [
			FAILED_TO_GET_ALLOCATOR = 0x00,
			ALLOC_NO_SPACE = 0x01,
			ALLOC_ZERO_CHUNK_FAILED = 0x02,
			ALLOC_TOO_BIG = 0x03,
			ALLOC_FAILED = 0x04,
		]
	}

	define_error! {
		module = module::BUFFER,
		errors = [
			BUFFER_FULL = 0x00,
			READ_KERNEL_FAILED = 0x01,
			READ_USER_FAILED = 0x02,
			READ_IOVEC_FAILED = 0x03,
		]
	}

	define_error! {
		module = module::MAP,
		errors = [
			MAP_INSERT_FAILED = 0x00,
			MAP_DELETE_FAILED = 0x01,
			MAP_GET_FAILED = 0x02,
		]
	}

	define_error! {
		module = module::CORE,
		errors = [
			CORE_READ_FAILED = 0x00,

			MISSING_IOVEC_IOV_BASE = 0x01,
			MISSING_IOVEC_IOVLEN = 0x02,
			MISSING_USER_MSGHDR_MSG_IOVLEN = 0x03,
			MISSING_USER_MSGHDR_MSG_IOV = 0x04,
			MISSING_MMSGHDR_MSG_HDR = 0x05,

			READ_SKC_IPV6ONLY_FAILED = 0x10,
			READ_SKC_FAMILY_FAILED = 0x11,
			READ_SKC_TYPE_FAILED = 0x12,
			READ_SKC_STATE_FAILED = 0x13,
			READ_FILE_FAILED = 0x14,
			READ_PRIVATE_DATA_FAILED = 0x15,
			READ_SK_FAILED = 0x16,
			READ_COPIED_SEQ_FAILED = 0x17,
			READ_WRITE_SEQ_FAILED = 0x18,
			READ_INET_SADDR_FAILED = 0x19,
			READ_SK_COMMON_FAILED = 0x1A,
			READ_SKC_DADDR_FAILED = 0x1B,
			READ_INET_SPORT_FAILED = 0x1C,
			READ_SKC_DPORT_FAILED = 0x1D,
			READ_TCP_SOCK_COPIED_SEQ_FAILED = 0x1E,
			READ_TCP_SOCK_WRITE_SEQ_FAILED = 0x1F,
		]
	}

	define_error! {
		module = module::PTR,
		errors = [
			FILE_PTR_NULL = 0x00
		]
	}

	define_error! {
		module = module::NETWORK,
		errors = [
			NOT_IP_PROTOCOL = 0x00,
			NOT_TCPUDP = 0x01,
			SOCKET_STATE_INVALID = 0x02,
			SOCKET_TYPE_NOT_SUPPORTED = 0x03,
		]
	}

	define_error! {
		module = module::PARSE,
		errors = [
			INFER_PAYLOAD_TOO_SHORT = 0x00,
			INFER_PAYLOAD_LENGTH_INVALID = 0x01,
			SOCKET_PROTOCOL_MISMATCH = 0x02,
			PREV_PAYLOAD_SAVED = 0x03,

			PARSE_CASSANDRA_FAILED = 0x10,
			PARSE_DNS_FAILED = 0x11,
			PARSE_HTTP1_FAILED = 0x12,
			PARSE_KAFKA_FAILED = 0x13,
			PARSE_MEMCACHED_FAILED = 0x14,
			PARSE_MONGODB_FAILED = 0x15,
			PARSE_MYSQL_FAILED = 0x16,
			PARSE_REDIS_FAILED = 0x17,
			PARSE_ROCKETMQ_FAILED = 0x18,
			PARSE_BINARY_THRIFT_FAILED = 0x19,
			PARSE_COMPACT_THRIFT_FAILED = 0x1A,

			DNS_QUESTION_NUM_INVALID = 0x20,
			DNS_RECOURSE_RECORD_NUM_INVALID = 0x21,
			DNS_OPCODE_PARSE_FAILED = 0x22,
			DNS_NOT_STANDARD_QUERY = 0x23,
			DNS_RCODE_PARSE_FAILED = 0x24,

			ROCKETMQ_PAYLOAD_LENGTH_INVALID = 0x30,
			ROCKETMQ_JSON_PARSE_FAILED = 0x31,
			ROCKETMQ_HEADER_LENGTH_INVALID = 0x32,
			ROCKETMQ_ROCKETMQ_PARSE_FAILED = 0x33,
			ROCKETMQ_TYPE_INVALID = 0x34,

			REDIS_PREFIX_INVALID = 0x40,
			REDIS_ERROR_PARSE_FAILED = 0x41,
			REDIS_CRLF_NOT_FOUND = 0x42,

			MYSQL_L4_PROTOCOL_INVALID = 0x50,
			MYSQL_PAYLOAD_LENGTH_INVALID = 0x51,
		]
	}
}

#[macro_export]
macro_rules! try_or_log {
	($ctx:expr, $expr:expr) => {
		match $expr {
			Ok(val) => val,
			Err(code) => {
				// Remove aya_log_ebpf::error! to avoid __bpf_trap
				aya_log_ebpf::debug!(
					$ctx,
					"ERROR: [{:X}:{:X}]",
					$crate::error::extract_module(code),
					$crate::error::extract_error(code)
				);
				return code;
			},
		}
	};
}
