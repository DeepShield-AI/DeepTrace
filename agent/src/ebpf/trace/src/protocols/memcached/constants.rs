use super::Memcached;
pub(super) const BINARY_PROTOCOL_REQUEST: u8 = 0x80;
pub(super) const BINARY_PROTOCOL_RESPONSE: u8 = 0x81;

pub(super) const HEADER_SIZE: usize = size_of::<Memcached>();
