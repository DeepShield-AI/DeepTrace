pub const MAX_PID_NUMBER: u32 = 256;
pub const MAX_INFER_PAYLOAD_SIZE: u32 = 1 << 6;
/// Maximum length of the 'struct iovec' array in a single call to readv or writev.
///
/// This macro has different values in different kernel versions.  The latest versions of the kernel
/// use 1024 and this is good choice.  Since the C library implementation of readv/writev is able to
/// emulate the functionality even if the currently running kernel does not support this large value
/// the readv/writev call will not fail because of this.
pub const IOV_MAX: usize = 1 << 3;

pub const IOVLEN_MAX: usize = 1;

pub const MAX_IOVEC_PAYLOAD_SIZE: u32 = 1 << 10;

pub(crate) mod mask {
	use super::{MAX_INFER_PAYLOAD_SIZE, MAX_IOVEC_PAYLOAD_SIZE};
	use trace_common::constants::MAX_PAYLOAD_SIZE;

	pub const PAYLOAD_MASK: u32 = MAX_PAYLOAD_SIZE - 1;
	pub const IOVEC_MASK: u32 = MAX_IOVEC_PAYLOAD_SIZE - 1;
	pub const INFER_MASK: u32 = MAX_INFER_PAYLOAD_SIZE - 1;
}
