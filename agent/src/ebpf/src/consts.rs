pub const MAX_PID_NUMBER: u32 = 256;
/// Maximum length of the 'struct iovec' array in a single call to readv or writev.
///
/// This macro has different values in different kernel versions.  The latest versions of the kernel
/// use 1024 and this is good choice.  Since the C library implementation of readv/writev is able to
/// emulate the functionality even if the currently running kernel does not support this large value
/// the readv/writev call will not fail because of this.
pub const IOV_MAX: usize = 1 << 3;

pub const IOVLEN_MAX: usize = 1;

pub const MAX_IOVEC_BUF_SIZE: u32 = 1 << 10; // must be double of MAX_PAYLOAD_SIZE
